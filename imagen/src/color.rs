use image::Rgba;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Represents a color in various formats with support for RGB, RGBA, and integer RGB values.
///
/// The enum supports three color formats:
/// - `Rgb`: RGB color with 8-bit components (r, g, b)
/// - `Rgba`: RGBA color with 8-bit RGB components and 32-bit float alpha (r, g, b, a)
/// - `RgbInt`: Single 32-bit integer containing RGB values
#[derive(Debug, Clone, Deserialize, Copy, Serialize, TS)]
#[serde(untagged)]
pub enum Color {
  /// RGB color with 8-bit components
  Rgb(u8, u8, u8),
  /// RGBA color with 8-bit RGB components and 32-bit float alpha
  Rgba(u8, u8, u8, f32),
  /// Single 32-bit integer containing RGB values
  RgbInt(u32),
}

impl Color {
  /// Returns the alpha value of the color as a float between 0.0 and 1.0.
  ///
  /// For RGB colors, returns 1.0 (fully opaque).
  /// For RGBA colors, returns the stored alpha value.
  pub fn alpha(&self) -> f32 {
    match self {
      Color::Rgba(_, _, _, a) => *a,
      _ => 1.0,
    }
  }

  /// Returns the alpha value as an 8-bit integer (0-255).
  ///
  /// Converts the float alpha value to an 8-bit integer representation.
  pub fn alpha_u8(&self) -> u8 {
    (self.alpha() * 255.0) as u8
  }
}

impl From<Color> for cosmic_text::Color {
  /// Converts a Color to a cosmic_text::Color.
  ///
  /// Handles all color formats and properly converts alpha values.
  fn from(color: Color) -> Self {
    match color {
      Color::Rgb(r, g, b) => cosmic_text::Color::rgb(r, g, b),
      Color::Rgba(r, g, b, a) => cosmic_text::Color::rgba(r, g, b, (a * 255.0) as u8),
      Color::RgbInt(rgb) => cosmic_text::Color(rgb),
    }
  }
}

impl Color {
  /// Returns the default alpha value (255) for RGB colors.
  fn default_alpha() -> u8 {
    255
  }
}

impl Default for Color {
  /// Returns a default black color (RGB: 0, 0, 0).
  fn default() -> Self {
    Color::Rgb(0, 0, 0)
  }
}

impl From<Color> for Rgba<u8> {
  /// Converts a Color to an `image::Rgba<u8>`.
  ///
  /// Handles all color formats and properly converts alpha values.
  /// For RgbInt, extracts RGB components using bit shifting.
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
