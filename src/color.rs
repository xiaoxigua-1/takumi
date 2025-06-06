use image::Rgba;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Copy)]
#[serde(untagged)]
pub enum Color {
  Rgb(u8, u8, u8),
  Rgba(u8, u8, u8, f32),
  RgbInt(u32),
}

impl From<Color> for cosmic_text::Color {
  fn from(color: Color) -> Self {
    match color {
      Color::Rgb(r, g, b) => cosmic_text::Color::rgb(r, g, b),
      Color::Rgba(r, g, b, a) => cosmic_text::Color::rgba(r, g, b, (a * 255.0) as u8),
      Color::RgbInt(rgb) => cosmic_text::Color(rgb),
    }
  }
}

impl Color {
  fn default_alpha() -> u8 {
    255
  }
}

impl Default for Color {
  fn default() -> Self {
    Color::Rgb(0, 0, 0)
  }
}

impl From<Color> for Rgba<u8> {
  fn from(color: Color) -> Self {
    match color {
      Color::Rgb(r, g, b) => Rgba([r, g, b, Color::default_alpha()]),
      Color::Rgba(r, g, b, a) => Rgba([r, g, b, (a * 255.0) as u8]),
      Color::RgbInt(rgb) => {
        let r = ((rgb >> 16) & 0xFF) as u8;
        let g = ((rgb >> 8) & 0xFF) as u8;
        let b = (rgb & 0xFF) as u8;

        Rgba([r, g, b, Color::default_alpha()])
      }
    }
  }
}
