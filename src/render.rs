use image::{ImageBuffer, RgbaImage};
use taffy::{AvailableSpace, NodeId, Style, TaffyTree, geometry::Size};

use crate::{
  color::Color,
  context::Context,
  node::{Node, NodeProperties, properties::ContainerProperties},
};

#[derive(Debug, Clone)]
pub struct DrawProps {
  pub background_color: Option<Color>,
}

#[derive(Debug, Clone)]
pub struct LayoutProps {
  pub width: u32,
  pub height: u32,
}

pub struct ImageRenderer {
  pub draw_props: DrawProps,
  pub layout_props: LayoutProps,
}

impl ImageRenderer {
  pub fn new(draw_props: DrawProps, layout_props: LayoutProps) -> Self {
    Self {
      draw_props,
      layout_props,
    }
  }

  pub fn create_taffy_tree(&self, nodes: Vec<Node>) -> (TaffyTree<Node>, NodeId) {
    let mut taffy = TaffyTree::new();

    let root_node = Node {
      properties: NodeProperties::Container(ContainerProperties { children: nodes }),
      background_color: None,
      border_color: None,
      style: Some(Style {
        size: Size {
          width: taffy::Dimension::Percent(1.0),
          height: taffy::Dimension::Percent(1.0),
        },
        ..Default::default()
      }),
    };

    let root_node_id = root_node.create_taffy_leaf(&mut taffy).unwrap();

    (taffy, root_node_id)
  }

  pub fn draw(
    &self,
    context: &Context,
    taffy: &mut TaffyTree<Node>,
    root_node_id: NodeId,
  ) -> RgbaImage {
    let mut canvas = ImageBuffer::from_pixel(
      self.layout_props.width,
      self.layout_props.height,
      self.draw_props.background_color.unwrap_or_default().into(),
    );

    let available_space = Size {
      width: AvailableSpace::Definite(self.layout_props.width as f32),
      height: AvailableSpace::Definite(self.layout_props.height as f32),
    };

    taffy.compute_layout(root_node_id, available_space).unwrap();

    draw_children(context, &mut canvas, taffy, root_node_id);

    canvas
  }
}

fn draw_children(
  context: &Context,
  canvas: &mut RgbaImage,
  taffy: &mut TaffyTree<Node>,
  node_id: NodeId,
) {
  for child_id in taffy.children(node_id).unwrap() {
    let child_layout = taffy.layout(child_id).unwrap();

    let node_kind = taffy.get_node_context(child_id).unwrap();

    node_kind.render(context, canvas, *child_layout);

    draw_children(context, canvas, taffy, child_id);
  }
}
