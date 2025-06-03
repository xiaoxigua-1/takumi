use image::RgbaImage;
use serde::Deserialize;
use taffy::{Layout, style::Style};

use crate::{
  context::Context,
  node::{Node, NodeKind},
};

#[derive(Deserialize, Clone, Debug)]
pub struct ContainerNode {
  pub children: Vec<NodeKind>,
  pub style: Style,
}

impl Node for ContainerNode {
  fn get_style(&self) -> Style {
    self.style.clone()
  }

  fn render(&self, context: &Context, canvas: &mut RgbaImage, layout: Layout) {
    for child in self.children.iter() {
      child.render(context, canvas, layout);
    }
  }
}
