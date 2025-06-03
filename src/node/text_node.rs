use ab_glyph::PxScale;
use image::RgbaImage;
use serde::Deserialize;
use taffy::{Layout, Style};

use crate::{color::Color, context::Context, node::Node};

#[derive(Debug, Clone, Deserialize)]
pub struct TextNode {
  pub content: String,
  pub font_size: Option<f32>,
  pub color: Option<Color>,
  pub style: Style,
}

impl Node for TextNode {
  fn get_style(&self) -> Style {
    self.style.clone()
  }

  fn render(&self, context: &Context, canvas: &mut RgbaImage, layout: Layout) {
    let color = self.color.clone().unwrap_or_default();
    let font_size = self.font_size.unwrap_or(16.0);
    let scale = PxScale::from(font_size);
  }
}
