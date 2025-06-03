use image::RgbaImage;
use imageproc::drawing::draw_filled_circle_mut;
use serde::Deserialize;
use taffy::{Layout, style::Style};

use crate::{color::Color, context::Context, node::Node};

#[derive(Debug, Clone, Deserialize)]
pub struct CircleNode {
  pub style: Style,
  pub color: Option<Color>,
}

impl Node for CircleNode {
  fn get_style(&self) -> Style {
    self.style.clone()
  }

  fn render(&self, _context: &Context, canvas: &mut RgbaImage, layout: Layout) {
    let color = self.color.unwrap_or_default();

    let size = (layout.size.width.min(layout.size.height) / 2.0) as i32;

    draw_filled_circle_mut(
      canvas,
      (
        layout.location.x as i32 + size,
        layout.location.y as i32 + size,
      ),
      size,
      color.into(),
    );
  }
}
