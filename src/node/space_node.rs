use image::RgbaImage;
use serde::Deserialize;
use taffy::Layout;
use taffy::style::Style;

use crate::context::Context;
use crate::node::Node;

#[derive(Debug, Clone, Deserialize)]
pub struct SpaceNode {
  pub style: Style,
}

impl Node for SpaceNode {
  fn get_style(&self) -> Style {
    self.style.clone()
  }

  fn render(&self, _context: &Context, _canvas: &mut RgbaImage, _layout: Layout) {}
}
