pub mod circle_node;
pub mod container_node;
pub mod image_node;
pub mod rect_node;
pub mod space_node;
pub mod text_node;

use std::fmt::Debug;

use image::RgbaImage;
use serde::Deserialize;
use taffy::{Layout, style::Style};

use crate::{
  context::Context,
  node::{
    circle_node::CircleNode, container_node::ContainerNode, image_node::ImageNode,
    rect_node::RectNode, space_node::SpaceNode, text_node::TextNode,
  },
};

pub trait Node: Debug {
  fn get_style(&self) -> Style {
    Style::default()
  }

  fn render(&self, context: &Context, canvas: &mut RgbaImage, layout: Layout);
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum NodeKind {
  #[serde(rename = "rect")]
  Rect(RectNode),
  #[serde(rename = "circle")]
  Circle(CircleNode),
  #[serde(rename = "text")]
  Text(TextNode),
  #[serde(rename = "image")]
  Image(ImageNode),
  #[serde(rename = "space")]
  Space(SpaceNode),
  #[serde(rename = "container")]
  Container(ContainerNode),
}

impl Node for NodeKind {
  fn get_style(&self) -> Style {
    match self {
      NodeKind::Rect(rect) => rect.get_style(),
      NodeKind::Circle(circle) => circle.get_style(),
      NodeKind::Text(text) => text.get_style(),
      NodeKind::Image(image) => image.get_style(),
      NodeKind::Space(space) => space.get_style(),
      NodeKind::Container(container) => container.get_style(),
    }
  }

  fn render(&self, context: &Context, canvas: &mut RgbaImage, layout: Layout) {
    match self {
      NodeKind::Rect(rect) => rect.render(context, canvas, layout),
      NodeKind::Circle(circle) => circle.render(context, canvas, layout),
      NodeKind::Text(text) => text.render(context, canvas, layout),
      NodeKind::Image(image) => image.render(context, canvas, layout),
      NodeKind::Space(space) => space.render(context, canvas, layout),
      NodeKind::Container(container) => container.render(context, canvas, layout),
    }
  }
}
