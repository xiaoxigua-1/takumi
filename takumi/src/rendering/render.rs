use std::{
  borrow::Cow,
  io::{Seek, Write},
  sync::mpsc::channel,
};

use image::{ExtendedColorType, ImageFormat, RgbaImage, codecs::jpeg::JpegEncoder};
use serde::{Deserialize, Serialize};
use taffy::{AvailableSpace, NodeId, Point, TaffyTree, geometry::Size};

use crate::{
  GlobalContext,
  layout::{
    Viewport,
    node::Node,
    style::{Angle, Transforms},
  },
  rendering::{Canvas, DEFAULT_SCALE, create_blocking_canvas_loop, draw_debug_border},
};

use crate::rendering::RenderContext;

/// Stores the context and node for rendering.
struct NodeContext<'ctx, N: Node<N>> {
  context: RenderContext<'ctx>,
  node: N,
}

/// Output format for the rendered image.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImageOutputFormat {
  /// WebP format, suitable for web images with good compression.
  WebP,
  /// AVIF format, even better compression than WebP, but requires more CPU time to encode.
  #[cfg(feature = "avif")]
  Avif,
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
      #[cfg(feature = "avif")]
      ImageOutputFormat::Avif => "image/avif",
      ImageOutputFormat::Png => "image/png",
      ImageOutputFormat::Jpeg => "image/jpeg",
    }
  }
}

impl From<ImageOutputFormat> for ImageFormat {
  fn from(format: ImageOutputFormat) -> Self {
    match format {
      ImageOutputFormat::WebP => Self::WebP,
      #[cfg(feature = "avif")]
      ImageOutputFormat::Avif => Self::Avif,
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
    _ => {
      image.write_to(destination, format.into())?;
    }
  }

  Ok(())
}

/// Renders a node to an image.
pub fn render<Nodes: Node<Nodes>>(
  viewport: Viewport,
  global: &GlobalContext,
  mut root_node: Nodes,
) -> Result<RgbaImage, crate::Error> {
  root_node.inherit_style_for_children();

  let mut taffy = TaffyTree::new();

  let (tx, rx) = channel();
  let canvas = Canvas::new(tx);

  let render_context = RenderContext {
    global,
    viewport,
    parent_font_size: viewport.font_size,
    scale: DEFAULT_SCALE,
    rotation: Angle::new(0.0),
  };

  let root_node_id = insert_taffy_node(&mut taffy, root_node, &render_context);

  let available_space = Size {
    width: AvailableSpace::Definite(viewport.width as f32),
    height: AvailableSpace::Definite(viewport.height as f32),
  };

  taffy
    .compute_layout_with_measure(
      root_node_id,
      available_space,
      |known_dimensions, available_space, _node_id, node_context, _style| {
        let node = node_context.unwrap();

        if let Size {
          width: Some(width),
          height: Some(height),
        } = known_dimensions
        {
          return Size { width, height };
        }

        node
          .node
          .measure(&node.context, available_space, known_dimensions)
      },
    )
    .unwrap();

  #[cfg(target_arch = "wasm32")]
  let canvas = {
    render_node(
      &taffy,
      root_node_id,
      &canvas,
      Point::ZERO,
      Transforms::default(),
    );

    drop(canvas);

    create_blocking_canvas_loop(viewport, rx)
  };

  #[cfg(not(target_arch = "wasm32"))]
  let canvas = {
    let handler = std::thread::spawn(move || create_blocking_canvas_loop(viewport, rx));

    render_node(
      &taffy,
      root_node_id,
      &canvas,
      Point::ZERO,
      Transforms::default(),
    );

    drop(canvas);

    handler.join().unwrap()
  };

  Ok(canvas)
}

fn render_node<Nodes: Node<Nodes>>(
  taffy: &TaffyTree<NodeContext<Nodes>>,
  node_id: NodeId,
  canvas: &Canvas,
  offset: Point<f32>,
  mut transform: Transforms,
) {
  let mut layout = *taffy.layout(node_id).unwrap();
  let node_context = taffy.get_node_context(node_id).unwrap();

  let mut render_context = node_context.context;

  layout.location.x += offset.x;
  layout.location.y += offset.y;

  // preserve the offset before the transform is applied
  let child_offset = layout.location;
  let style = node_context.node.get_style();

  if let Some(node_transform) = &style.transform {
    let mut node_transform = Cow::Borrowed(node_transform);

    if let Some(transform_origin) = &style.transform_origin {
      node_transform = Cow::Owned(node_transform.with_transform_origin(transform_origin));
    }

    transform.chain(&node_transform);
  }

  transform.apply(&mut render_context, &mut layout);

  node_context
    .node
    .draw_on_canvas(&render_context, canvas, layout);

  if node_context.context.global.draw_debug_border {
    draw_debug_border(canvas, layout, *render_context.rotation);
  }

  for child_id in taffy.children(node_id).unwrap() {
    render_node(taffy, child_id, canvas, child_offset, transform.clone());
  }
}

fn insert_taffy_node<'ctx, Nodes: Node<Nodes>>(
  taffy: &mut TaffyTree<NodeContext<'ctx, Nodes>>,
  mut node: Nodes,
  render_context: &RenderContext<'ctx>,
) -> NodeId {
  let children = node.take_children();

  let parent_font_size = node
    .get_style()
    .inheritable_style
    .font_size
    .map(|f| f.resolve_to_px(render_context, render_context.parent_font_size))
    .unwrap_or(render_context.parent_font_size);

  let node_id = taffy
    .new_leaf_with_context(
      node.get_style().resolve_to_taffy_style(render_context),
      NodeContext {
        context: *render_context,
        node,
      },
    )
    .unwrap();

  if let Some(children) = children {
    let render_context = RenderContext {
      parent_font_size,
      ..*render_context
    };

    let children_ids = children
      .into_iter()
      .map(|child| insert_taffy_node(taffy, child, &render_context))
      .collect::<Vec<_>>();

    taffy.set_children(node_id, &children_ids).unwrap();
  }

  node_id
}
