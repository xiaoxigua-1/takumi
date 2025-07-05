use std::io::{Seek, Write};

use image::{ImageFormat, RgbImage, RgbaImage, codecs::jpeg::JpegEncoder};
use serde::{Deserialize, Serialize};
use slotmap::{DefaultKey, KeyData, SecondaryMap};
use taffy::{AvailableSpace, NodeId, Point, TaffyTree, geometry::Size};
use thiserror::Error;

use crate::{
  core::{GlobalContext, Viewport},
  layout::Node,
  rendering::{FastBlendImage, draw_debug_border},
};

use crate::core::RenderContext;

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
      let rgb_image = RgbImage::from_par_fn(image.width(), image.height(), |x, y| {
        let pixel = image.get_pixel(x, y);
        image::Rgb([pixel[0], pixel[1], pixel[2]])
      });

      let mut encoder = JpegEncoder::new_with_quality(destination, jpeg_quality.unwrap_or(75));

      encoder.encode_image(&rgb_image)?;
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
#[derive(Error, Debug)]
pub enum RenderError {
  /// The Taffy context is missing, should call `construct_taffy_tree` first.
  #[error("Missing Taffy context, should call `construct_taffy_tree` first.")]
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

    let render_context = RenderContext {
      global,
      viewport,
      parent_font_size: viewport.font_size,
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

    if render_context.global.print_debug_tree {
      taffy_context.taffy.print_tree(taffy_context.root_node_id);
    }

    draw_node_with_layout(
      taffy_context,
      global,
      viewport,
      &mut canvas,
      taffy_context.root_node_id,
      Point::zero(),
    );

    Ok(canvas.0)
  }

  fn get_taffy_context_mut(&mut self) -> Result<&mut TaffyContext<Nodes>, RenderError> {
    self
      .taffy_context
      .as_mut()
      .ok_or(RenderError::TaffyContextMissing)
  }
}

fn draw_node_with_layout<Nodes: Node<Nodes>>(
  taffy_context: &TaffyContext<Nodes>,
  global: &GlobalContext,
  viewport: Viewport,
  canvas: &mut FastBlendImage,
  node_id: NodeId,
  relative_offset: Point<f32>,
) {
  let node_render = taffy_context
    .node_map
    .get(KeyData::from_ffi(node_id.into()).into())
    .unwrap();

  let mut node_layout = *taffy_context.taffy.layout(node_id).unwrap();

  node_layout.location.x += relative_offset.x;
  node_layout.location.y += relative_offset.y;

  let render_context = RenderContext {
    global,
    viewport,
    parent_font_size: node_render.parent_font_size,
  };

  node_render
    .node
    .draw_on_canvas(&render_context, canvas, node_layout);

  if global.draw_debug_border {
    draw_debug_border(canvas, node_layout);
  }

  for child in taffy_context.taffy.children(node_id).unwrap() {
    draw_node_with_layout(
      taffy_context,
      global,
      viewport,
      canvas,
      child,
      node_layout.location,
    );
  }
}
