use image::Rgba;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Copy)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  #[serde(default = "Color::default_alpha")]
  pub a: u8,
}

impl Color {
  fn default_alpha() -> u8 {
    255
  }
}

impl Default for Color {
  fn default() -> Self {
    Color {
      r: 0,
      g: 0,
      b: 0,
      a: Self::default_alpha(),
    }
  }
}

impl From<Color> for Rgba<u8> {
  fn from(color: Color) -> Self {
    Rgba([color.r, color.g, color.b, color.a])
  }
}
