use image::{ImageBuffer, RgbaImage};
use slotmap::{DefaultKey, KeyData, SecondaryMap};
use taffy::{AvailableSpace, NodeId, Point, TaffyTree, geometry::Size};

use crate::{
  context::Context,
  node::{
    Node,
    draw::{FastBlendImage, draw_debug_border},
  },
};

/// A renderer for creating images from a container node with specified dimensions.
///
/// The renderer takes a root container node and uses Taffy for layout calculations
/// to render the final image with the specified content dimensions.
pub struct ImageRenderer<Nodes: Node<Nodes>> {
  content_width: u32,
  content_height: u32,
  taffy_context: Option<TaffyContext<Nodes>>,
}

struct TaffyContext<Nodes: Node<Nodes>> {
  taffy: TaffyTree<()>,
  root_node_id: NodeId,
  node_map: SecondaryMap<DefaultKey, Nodes>,
}

impl<Nodes: Node<Nodes>> ImageRenderer<Nodes> {
  /// Creates a new ImageRenderer with the specified dimensions.
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      content_width: width,
      content_height: height,
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
  node_map: &mut SecondaryMap<DefaultKey, Nodes>,
  node: Nodes,
) -> NodeId {
  let node_id = taffy.new_leaf(node.get_style().clone().into()).unwrap();

  if let Some(children) = &node.get_children() {
    let children_ids = children
      .iter()
      .map(|child| insert_taffy_node(taffy, node_map, (*child).clone()))
      .collect::<Vec<_>>();

    taffy.set_children(node_id, &children_ids).unwrap();
  }

  node_map.insert(KeyData::from_ffi(node_id.into()).into(), node);

  node_id
}

impl<Nodes: Node<Nodes>> ImageRenderer<Nodes> {
  /// Creates a new TaffyTree with the root node and returns both the tree and root node ID.
  pub fn construct_taffy_tree(&mut self, root_node: Nodes) {
    let mut taffy = TaffyTree::new();

    let mut node_map = SecondaryMap::new();

    let root_node_id = insert_taffy_node(&mut taffy, &mut node_map, root_node);

    self.taffy_context = Some(TaffyContext {
      taffy,
      root_node_id,
      node_map,
    });
  }

  /// Renders the image using the provided context and TaffyTree.
  pub fn draw(&mut self, context: &Context) -> Result<RgbaImage, RenderError> {
    let mut canvas = FastBlendImage(ImageBuffer::new(self.content_width, self.content_height));

    let available_space = Size {
      width: AvailableSpace::Definite(self.content_width as f32),
      height: AvailableSpace::Definite(self.content_height as f32),
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

          node.measure(context, available_space, known_dimensions)
        },
      )
      .unwrap();

    if context.print_debug_tree {
      taffy_context.taffy.print_tree(taffy_context.root_node_id);
    }

    draw_node_with_layout(
      taffy_context,
      context,
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
  context: &Context,
  canvas: &mut FastBlendImage,
  node_id: NodeId,
  relative_offset: Point<f32>,
) {
  let node = taffy_context
    .node_map
    .get(KeyData::from_ffi(node_id.into()).into())
    .unwrap();

  let mut node_layout = *taffy_context.taffy.layout(node_id).unwrap();

  node_layout.location.x += relative_offset.x;
  node_layout.location.y += relative_offset.y;

  node.draw_on_canvas(context, canvas, node_layout);

  if context.draw_debug_border {
    draw_debug_border(canvas, node_layout);
  }

  for child in taffy_context.taffy.children(node_id).unwrap() {
    draw_node_with_layout(taffy_context, context, canvas, child, node_layout.location);
  }
}
