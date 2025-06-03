use image::RgbaImage;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use serde::Deserialize;
use taffy::{Layout, style::Style};

use crate::{color::Color, context::Context, node::Node};

#[derive(Debug, Clone, Deserialize)]
pub struct RectNode {
  pub style: Style,
  pub color: Option<Color>,
}

impl Node for RectNode {
  fn get_style(&self) -> Style {
    self.style.clone()
  }

  fn render(&self, _context: &Context, canvas: &mut RgbaImage, layout: Layout) {
    let color = self.color.unwrap_or_default();
    let rect = Rect::at(layout.location.x as i32, layout.location.y as i32)
      .of_size(layout.size.width as u32, layout.size.height as u32);

    draw_filled_rect_mut(canvas, rect, color.into());
  }
}
