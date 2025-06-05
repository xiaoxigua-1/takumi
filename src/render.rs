use image::{ImageBuffer, RgbaImage};
use taffy::{AvailableSpace, NodeId, Point, TaffyTree, geometry::Size};

use crate::{
  context::Context,
  node::{
    Node,
    style::{Length, ValueOrAutoFull},
  },
};

pub struct ImageRenderer {
  pub root_node: Node,
  content_width: u32,
  content_height: u32,
}

#[derive(Debug)]
pub enum ImageRendererError {
  InvalidContentSize,
}

impl TryFrom<Node> for ImageRenderer {
  type Error = ImageRendererError;

  fn try_from(value: Node) -> Result<Self, Self::Error> {
    let Some(ValueOrAutoFull::SpecificValue(Length(width))) = value.style.width else {
      return Err(ImageRendererError::InvalidContentSize);
    };

    let Some(ValueOrAutoFull::SpecificValue(Length(height))) = value.style.height else {
      return Err(ImageRendererError::InvalidContentSize);
    };

    Ok(Self {
      root_node: value,
      content_width: width as u32,
      content_height: height as u32,
    })
  }
}

impl ImageRenderer {
  pub fn create_taffy_tree(&self) -> (TaffyTree<Node>, NodeId) {
    let mut taffy = TaffyTree::new();

    let root_node_id = self
      .root_node
      .clone()
      .create_taffy_leaf(&mut taffy)
      .unwrap();

    (taffy, root_node_id)
  }

  pub fn draw(
    &self,
    context: &Context,
    taffy: &mut TaffyTree<Node>,
    root_node_id: NodeId,
  ) -> RgbaImage {
    let mut canvas = ImageBuffer::new(self.content_width, self.content_height);

    let available_space = Size {
      width: AvailableSpace::Definite(self.content_width as f32),
      height: AvailableSpace::Definite(self.content_height as f32),
    };

    taffy
      .compute_layout_with_measure(
        root_node_id,
        available_space,
        |known_dimensions, available_space, _node_id, node_context, _style| {
          let Some(node) = node_context else {
            return Size::ZERO;
          };

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
      taffy.print_tree(root_node_id);
    }

    draw_from_node_id_with_layout(context, &mut canvas, taffy, root_node_id, Point::zero());

    canvas
  }
}

fn draw_from_node_id_with_layout(
  context: &Context,
  canvas: &mut RgbaImage,
  taffy: &mut TaffyTree<Node>,
  node_id: NodeId,
  relative_offset: Point<f32>,
) {
  let mut node_layout = *taffy.layout(node_id).unwrap();

  node_layout.location.x += relative_offset.x;
  node_layout.location.y += relative_offset.y;

  let node_kind = taffy.get_node_context(node_id).unwrap();

  node_kind.render(context, canvas, node_layout);

  for child_id in taffy.children(node_id).unwrap() {
    draw_from_node_id_with_layout(
      context,
      canvas,
      taffy,
      child_id,
      node_layout.location,
    );
  }
}
