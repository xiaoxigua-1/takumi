use std::io::{Seek, Write};

use image::{ExtendedColorType, ImageFormat, RgbaImage, codecs::jpeg::JpegEncoder};
use serde::{Deserialize, Serialize};
use slotmap::{DefaultKey, KeyData, SecondaryMap};
use taffy::{AvailableSpace, Layout, NodeId, Point, TaffyTree, geometry::Size};

use crate::{
  GlobalContext,
  layout::{Viewport, node::Node},
  rendering::{BoxShadowResolved, FastBlendImage, draw_debug_border},
};

use crate::rendering::RenderContext;

/// A renderer for creating images from a container node with specified dimensions.
///
/// The renderer takes a root container node and uses Taffy for layout calculations
/// to render the final image with the specified content dimensions.
pub struct ImageRenderer<Nodes: Node<Nodes>> {
  viewport: Viewport,
  taffy_context: Option<TaffyContext<Nodes>>,
}

/// A renderer for a single node.
///
/// This renderer is used to render a single node with the specified dimensions.
/// It is used to render the node with the specified dimensions.
struct NodeRender<Nodes: Node<Nodes>> {
  node: Nodes,
  parent_font_size: f32,
}

struct TaffyContext<Nodes: Node<Nodes>> {
  taffy: TaffyTree<()>,
  root_node_id: NodeId,
  node_map: SecondaryMap<DefaultKey, NodeRender<Nodes>>,
}

/// Output format for the rendered image.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub enum ImageOutputFormat {
  /// WebP format, suitable for web images with good compression.
  WebP,
  /// PNG format, lossless and supports transparency.
  Png,
  /// JPEG format, lossy compression suitable for photographs.
  Jpeg,
}

impl ImageOutputFormat {
  /// Returns the MIME type for the image output format.
  pub fn content_type(&self) -> &'static str {
    match self {
      ImageOutputFormat::WebP => "image/webp",
      ImageOutputFormat::Png => "image/png",
      ImageOutputFormat::Jpeg => "image/jpeg",
    }
  }
}

impl From<ImageOutputFormat> for ImageFormat {
  fn from(format: ImageOutputFormat) -> Self {
    match format {
      ImageOutputFormat::WebP => Self::WebP,
      ImageOutputFormat::Png => Self::Png,
      ImageOutputFormat::Jpeg => Self::Jpeg,
    }
  }
}

/// Writes the rendered image to the specified destination.
pub fn write_image<T: Write + Seek>(
  image: &RgbaImage,
  destination: &mut T,
  format: ImageOutputFormat,
  jpeg_quality: Option<u8>,
) -> Result<(), image::ImageError> {
  match format {
    ImageOutputFormat::Png | ImageOutputFormat::WebP => {
      image.write_to(destination, format.into())?;
    }
    ImageOutputFormat::Jpeg => {
      // Strip alpha channel into a tightly packed RGB buffer
      let raw = image.as_raw();
      let mut rgb = Vec::with_capacity(raw.len() / 4 * 3);
      for px in raw.chunks_exact(4) {
        rgb.extend_from_slice(&px[..3]);
      }

      let mut encoder = JpegEncoder::new_with_quality(destination, jpeg_quality.unwrap_or(75));
      encoder.encode(&rgb, image.width(), image.height(), ExtendedColorType::Rgb8)?;
    }
  }

  Ok(())
}

impl<Nodes: Node<Nodes>> ImageRenderer<Nodes> {
  /// Creates a new ImageRenderer with the specified dimensions.
  pub fn new(viewport: Viewport) -> Self {
    Self {
      viewport,
      taffy_context: None,
    }
  }
}

/// An error that can occur when rendering an image.
#[derive(Debug)]
pub enum RenderError {
  /// The Taffy context is missing, should call `construct_taffy_tree` first.
  TaffyContextMissing,
}

fn insert_taffy_node<Nodes: Node<Nodes>>(
  taffy: &mut TaffyTree<()>,
  node_map: &mut SecondaryMap<DefaultKey, NodeRender<Nodes>>,
  node: Nodes,
  render_context: &RenderContext,
) -> NodeId {
  let style = node.get_style();

  let node_id = taffy
    .new_leaf(style.resolve_to_taffy_style(render_context))
    .unwrap();

  if let Some(children) = &node.get_children() {
    let render_context = RenderContext {
      global: render_context.global,
      viewport: render_context.viewport,
      parent_font_size: style
        .inheritable_style
        .font_size
        .map(|f| f.resolve_to_px(render_context))
        .unwrap_or(render_context.parent_font_size),
    };

    let children_ids = children
      .iter()
      .map(|child| insert_taffy_node(taffy, node_map, (*child).clone(), &render_context))
      .collect::<Vec<_>>();

    taffy.set_children(node_id, &children_ids).unwrap();
  }

  node_map.insert(
    KeyData::from_ffi(node_id.into()).into(),
    NodeRender {
      node,
      parent_font_size: render_context.parent_font_size,
    },
  );

  node_id
}

impl<Nodes: Node<Nodes>> ImageRenderer<Nodes> {
  /// Creates a new TaffyTree with the root node and returns both the tree and root node ID.
  pub fn construct_taffy_tree(&mut self, root_node: Nodes, global: &GlobalContext) {
    let mut taffy = TaffyTree::new();

    let mut node_map = SecondaryMap::new();

    let render_context = RenderContext {
      global,
      viewport: self.viewport,
      parent_font_size: self.viewport.font_size,
    };

    let root_node_id = insert_taffy_node(&mut taffy, &mut node_map, root_node, &render_context);

    self.taffy_context = Some(TaffyContext {
      taffy,
      root_node_id,
      node_map,
    });
  }

  /// Renders the image using the provided context and TaffyTree.
  pub fn draw(&mut self, global: &GlobalContext) -> Result<RgbaImage, RenderError> {
    let viewport = self.viewport;
    let mut canvas = FastBlendImage(RgbaImage::new(viewport.width, viewport.height));

    let available_space = Size {
      width: AvailableSpace::Definite(viewport.width as f32),
      height: AvailableSpace::Definite(viewport.height as f32),
    };

    let taffy_context = self.get_taffy_context_mut()?;

    taffy_context
      .taffy
      .compute_layout_with_measure(
        taffy_context.root_node_id,
        available_space,
        |known_dimensions, available_space, node_id, _node_context, _style| {
          let node = taffy_context
            .node_map
            .get(KeyData::from_ffi(node_id.into()).into())
            .unwrap();

          if let Size {
            width: Some(width),
            height: Some(height),
          } = known_dimensions
          {
            return Size { width, height };
          }

          let render_context = RenderContext {
            global,
            viewport,
            parent_font_size: node.parent_font_size,
          };
          node
            .node
            .measure(&render_context, available_space, known_dimensions)
        },
      )
      .unwrap();

    // When the `rayon` feature is enabled, draw nodes in parallel onto minimal per-node canvases
    // and then composite them back to the main canvas in the original paint order.
    // Fallback to the original sequential renderer otherwise.

    // Collect paint tasks in paint order (preorder: parent then children)
    let tasks = collect_paint_tasks(taffy_context);

    // Fast path: single node can draw directly onto the main canvas to preserve
    // exact pixel parity with the legacy sequential renderer.
    if tasks.len() == 1 {
      let t = &tasks[0];
      let render_context = RenderContext {
        global,
        viewport,
        parent_font_size: t.parent_font_size,
      };

      t.node
        .draw_on_canvas(&render_context, &mut canvas, t.layout);

      if global.draw_debug_border {
        draw_debug_border(&mut canvas, t.layout);
      }

      return Ok(canvas.0);
    }

    // Render each task on a minimal canvas
    #[cfg(feature = "rayon")]
    let jobs: Vec<PaintJob> = {
      use rayon::prelude::*;
      tasks
        .par_iter()
        .map(|task| render_task_to_job(task, global, viewport))
        .collect()
    };

    #[cfg(not(feature = "rayon"))]
    let jobs: Vec<PaintJob> = tasks
      .iter()
      .map(|task| render_task_to_job(task, global, viewport))
      .collect();

    // Composite in order to preserve stacking
    for job in jobs {
      canvas.overlay_image(&job.image, job.overlay_left, job.overlay_top);
    }

    Ok(canvas.0)
  }

  fn get_taffy_context_mut(&mut self) -> Result<&mut TaffyContext<Nodes>, RenderError> {
    self
      .taffy_context
      .as_mut()
      .ok_or(RenderError::TaffyContextMissing)
  }
}

/// A collected paint task representing a single node draw with absolute layout.
struct PaintTask<Nodes: Node<Nodes>> {
  node: Nodes,
  layout: Layout,
  parent_font_size: f32,
}

/// A rendered job result with an image and its overlay position.
struct PaintJob {
  image: RgbaImage,
  overlay_left: u32,
  overlay_top: u32,
}

fn collect_paint_tasks<Nodes: Node<Nodes>>(
  taffy_context: &TaffyContext<Nodes>,
) -> Vec<PaintTask<Nodes>> {
  let mut tasks: Vec<PaintTask<Nodes>> = Vec::new();

  fn walk<Nodes: Node<Nodes>>(
    taffy_context: &TaffyContext<Nodes>,
    node_id: NodeId,
    accumulated_offset: Point<f32>,
    tasks: &mut Vec<PaintTask<Nodes>>,
  ) {
    let node_render = taffy_context
      .node_map
      .get(KeyData::from_ffi(node_id.into()).into())
      .unwrap();

    let mut layout = *taffy_context.taffy.layout(node_id).unwrap();
    layout.location.x += accumulated_offset.x;
    layout.location.y += accumulated_offset.y;

    tasks.push(PaintTask {
      node: node_render.node.clone(),
      layout,
      parent_font_size: node_render.parent_font_size,
    });

    for child in taffy_context.taffy.children(node_id).unwrap() {
      walk(taffy_context, child, layout.location, tasks);
    }
  }

  walk(
    taffy_context,
    taffy_context.root_node_id,
    Point::zero(),
    &mut tasks,
  );

  tasks
}

fn render_task_to_job<Nodes: Node<Nodes>>(
  task: &PaintTask<Nodes>,
  global: &GlobalContext,
  viewport: Viewport,
) -> PaintJob {
  let render_context = RenderContext {
    global,
    viewport,
    parent_font_size: task.parent_font_size,
  };

  // Compute minimal integer canvas bounds matching runtime truncation behavior
  let (overlay_left, overlay_top, width, height) =
    compute_paint_canvas_bounds(&task.node, &task.layout, &render_context);

  // Create minimal canvas
  let mut local_canvas = FastBlendImage(RgbaImage::new(width, height));

  // Shift layout into local space
  let mut local_layout = task.layout;
  local_layout.location.x -= overlay_left as f32;
  local_layout.location.y -= overlay_top as f32;

  // Draw node
  task
    .node
    .draw_on_canvas(&render_context, &mut local_canvas, local_layout);

  // Optional debug border
  if global.draw_debug_border {
    draw_debug_border(&mut local_canvas, local_layout);
  }

  PaintJob {
    image: local_canvas.0,
    overlay_left,
    overlay_top,
  }
}

type UiRect = taffy::Rect<u32>;

fn rect_from_layout_base(layout: &Layout) -> UiRect {
  let left = clamp_trunc_to_u32(layout.location.x);
  let top = clamp_trunc_to_u32(layout.location.y);
  let right = left + (layout.size.width.max(0.0) as u32);
  let bottom = top + (layout.size.height.max(0.0) as u32);

  UiRect {
    left,
    right,
    top,
    bottom,
  }
}

fn rect_union_in_place(a: &mut UiRect, b: &UiRect) {
  if b.left < a.left {
    a.left = b.left;
  }
  if b.top < a.top {
    a.top = b.top;
  }
  if b.right > a.right {
    a.right = b.right;
  }
  if b.bottom > a.bottom {
    a.bottom = b.bottom;
  }
}

#[inline]
fn rect_width_height(rect: &UiRect) -> (u32, u32) {
  (
    rect.right.saturating_sub(rect.left),
    rect.bottom.saturating_sub(rect.top),
  )
}

#[inline]
fn clamp_trunc_to_u32(value: f32) -> u32 {
  if value < 0.0 { 0 } else { value as u32 }
}

fn compute_outset_shadow_rect(resolved: &BoxShadowResolved, layout: &Layout) -> UiRect {
  let blur = resolved.blur_radius.max(0.0);
  let spread = resolved.spread_radius;
  let inflate = (blur + spread) * 2.0;

  let shadow_w = (layout.size.width + inflate).max(0.0) as u32;
  let shadow_h = (layout.size.height + inflate).max(0.0) as u32;

  let left = clamp_trunc_to_u32(layout.location.x + (resolved.offset_x - blur - spread));
  let top = clamp_trunc_to_u32(layout.location.y + (resolved.offset_y - blur - spread));

  UiRect {
    left,
    top,
    right: left + shadow_w,
    bottom: top + shadow_h,
  }
}

fn compute_paint_canvas_bounds<Nodes: Node<Nodes>>(
  node: &Nodes,
  layout: &Layout,
  context: &RenderContext,
) -> (u32, u32, u32, u32) {
  // Base bounds
  let mut bounds = rect_from_layout_base(layout);

  if let Some(shadows) = &node.get_style().box_shadow {
    for shadow in &shadows.0 {
      if shadow.inset {
        continue;
      }

      let resolved = BoxShadowResolved::from_box_shadow(shadow, context);
      let shadow_rect = compute_outset_shadow_rect(&resolved, layout);

      rect_union_in_place(&mut bounds, &shadow_rect);
    }
  }

  let (width, height) = rect_width_height(&bounds);
  (bounds.left, bounds.top, width, height)
}
