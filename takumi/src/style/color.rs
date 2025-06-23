use image::Rgba;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Represents a color input that can be either a color or a gradient.
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[serde(untagged)]
pub enum ColorInput {
  /// A single color
  Color(Color),
  /// A gradient with color steps and an angle for directional gradients
  Gradient(Gradient),
}

impl ColorAt for ColorInput {
  fn at(&self, width: f32, height: f32, x: u32, y: u32) -> Color {
    match self {
      ColorInput::Color(color) => color.at(width, height, x, y),
      ColorInput::Gradient(gradient) => gradient.at(width, height, x, y),
    }
  }
}

impl Default for ColorInput {
  fn default() -> Self {
    ColorInput::Color(Color::default())
  }
}

impl ColorInput {
  /// Determines if the color input is fully transparent.
  ///
  /// # Returns
  /// `true` if the color input is transparent; otherwise, `false`.
  #[must_use]
  pub fn is_transparent(&self) -> bool {
    match self {
      ColorInput::Color(color) => color.is_transparent(),
      ColorInput::Gradient(gradient) => gradient.stops.iter().all(Color::is_transparent),
    }
  }
}

/// Represents a gradient with color steps and an angle for directional gradients.
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct Gradient {
  /// The color stops that make up the gradient
  pub stops: Vec<Color>,
  /// The angle in degrees for the gradient direction (0-360)
  pub angle: f32,
}

impl From<Color> for ColorInput {
  fn from(color: Color) -> Self {
    ColorInput::Color(color)
  }
}

/// A trait for calculating the color at a specific position within a color input.
pub trait ColorAt {
  /// Calculates the color at a specific position within the gradient.
  ///
  /// # Returns
  /// The interpolated color at the given position.
  fn at(&self, width: f32, height: f32, x: u32, y: u32) -> Color;
}

impl ColorAt for Gradient {
  fn at(&self, width: f32, height: f32, x: u32, y: u32) -> Color {
    if self.stops.is_empty() {
      return Color::default();
    }

    if self.stops.len() == 1 {
      return self.stops[0];
    }

    let angle_rad = self.angle.to_radians();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    let center_x = width / 2.0;
    let center_y = height / 2.0;

    let relative_x = x as f32 - center_x;
    let relative_y = y as f32 - center_y;

    let gradient_length = (width.abs() + height.abs()) / 2.0;
    let projection = (relative_x * cos_angle + relative_y * sin_angle) / gradient_length;
    let normalized_position = (projection + 1.0) / 2.0;
    let clamped_position = normalized_position.clamp(0.0, 1.0);

    let step_size = 1.0 / (self.stops.len() - 1) as f32;
    let step_index = (clamped_position / step_size).floor() as usize;
    let step_index = step_index.min(self.stops.len() - 2);

    let local_t = (clamped_position - step_index as f32 * step_size) / step_size;

    let color1 = self.stops[step_index];
    let color2 = self.stops[step_index + 1];

    self.interpolate_colors(color1, color2, local_t)
  }
}

impl Gradient {
  fn interpolate_colors(&self, color1: Color, color2: Color, t: f32) -> Color {
    let (r1, g1, b1, a1) = color1.into();
    let (r2, g2, b2, a2) = color2.into();

    let r = (r1 as f32 * (1.0 - t) + r2 as f32 * t) as u8;
    let g = (g1 as f32 * (1.0 - t) + g2 as f32 * t) as u8;
    let b = (b1 as f32 * (1.0 - t) + b2 as f32 * t) as u8;
    let a = a1 * (1.0 - t) + a2 * t;

    if a < 1.0 {
      Color::Rgba(r, g, b, a)
    } else {
      Color::Rgb(r, g, b)
    }
  }
}

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
  /// RGBA color with 8-bit RGB components and 32-bit float alpha (alpha is between 0.0 and 1.0)
  Rgba(u8, u8, u8, f32),
  /// Single 32-bit integer containing RGB values
  RgbInt(u32),
}

impl Color {
  /// Returns the alpha value of the color as a float between 0.0 and 1.0.
  ///
  /// For RGB colors, returns 1.0 (fully opaque).
  /// For RGBA colors, returns the stored alpha value.
  #[must_use]
  pub const fn alpha(&self) -> f32 {
    match self {
      Color::Rgba(_, _, _, a) => *a,
      _ => 1.0,
    }
  }

  /// Returns the alpha value as an 8-bit integer (0-255).
  ///
  /// Converts the float alpha value to an 8-bit integer representation.
  #[must_use]
  pub fn alpha_u8(&self) -> u8 {
    (self.alpha() * 255.0) as u8
  }

  /// Determines if the color is fully transparent.
  ///
  /// # Returns
  /// `true` if the color is transparent; otherwise, `false`.
  #[must_use]
  pub fn is_transparent(&self) -> bool {
    self.alpha() == 0.0
  }
}

impl ColorAt for Color {
  fn at(&self, _width: f32, _height: f32, _x: u32, _y: u32) -> Color {
    *self
  }
}

impl From<Color> for cosmic_text::Color {
  /// Converts a Color to a `cosmic_text::Color`.
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

impl Default for Color {
  /// Returns a default black color (RGB: 0, 0, 0).
  fn default() -> Self {
    Color::Rgb(0, 0, 0)
  }
}

impl From<Color> for (u8, u8, u8, f32) {
  fn from(color: Color) -> Self {
    match color {
      Color::Rgb(r, g, b) => (r, g, b, 1.0),
      Color::Rgba(r, g, b, a) => (r, g, b, a),
      Color::RgbInt(rgb) => {
        let r = ((rgb >> 16) & 0xFF) as u8;
        let g = ((rgb >> 8) & 0xFF) as u8;
        let b = (rgb & 0xFF) as u8;
        (r, g, b, 1.0)
      }
    }
  }
}

impl From<Color> for Rgba<u8> {
  /// Converts a Color to an `image::Rgba<u8>`.
  ///
  /// Handles all color formats and properly converts alpha values.
  /// For `RgbInt`, extracts RGB components using bit shifting.
  fn from(color: Color) -> Self {
    let (r, g, b, a) = color.into();
    Rgba([r, g, b, (a * 255.0) as u8])
  }
}
