use image::Rgba;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Represents a color input that can be either a color or a gradient.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
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
      ColorInput::Gradient(gradient) => gradient
        .stops
        .iter()
        .all(|stop| stop.color.is_transparent()),
    }
  }
}

/// Represents a single color stop in a gradient.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
pub struct GradientStop {
  /// The color of the gradient stop
  pub color: Color,
  /// Position in the range [0.0, 1.0]
  pub position: f32,
}

/// Represents a gradient with color steps and an angle for directional gradients.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
pub struct Gradient {
  /// The color stops that make up the gradient
  pub stops: Vec<GradientStop>,
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
    let stops_size = self.stops.len();

    if stops_size == 0 {
      return Color::default();
    }

    if stops_size == 1 {
      return self.stops[0].color;
    }

    // For a horizontal gradient (angle 0), map x-coordinate directly
    // For other angles, use projection method
    let normalized_position = if self.angle == 0.0 && width > 0.0 {
      // Simple horizontal gradient mapping
      (x as f32) / (width - 1.0)
    } else if self.angle == 180.0 && width > 0.0 {
      // Simple horizontal gradient mapping (reversed)
      1.0 - (x as f32) / (width - 1.0)
    } else if self.angle == 90.0 && height > 0.0 {
      // Simple vertical gradient mapping
      (y as f32) / (height - 1.0)
    } else if self.angle == 270.0 && height > 0.0 {
      // Simple vertical gradient mapping (reversed)
      1.0 - (y as f32) / (height - 1.0)
    } else {
      // Convert angle to standard mathematical convention (0° = along positive x-axis)
      let angle_rad = self.angle.to_radians();
      let cos_angle = angle_rad.cos();
      let sin_angle = angle_rad.sin();

      // Calculate relative position from center
      let center_x = width / 2.0;
      let center_y = height / 2.0;
      let relative_x = x as f32 - center_x;
      let relative_y = y as f32 - center_y;

      // Project the relative position onto the gradient direction vector
      let projection = relative_x * cos_angle + relative_y * sin_angle;

      // Determine the maximum projection distance based on angle and dimensions
      let max_projection = {
        // Calculate projections of the corners relative to center
        let corners = [
          (-center_x, -center_y),                // Top-left
          (width - center_x, -center_y),         // Top-right
          (-center_x, height - center_y),        // Bottom-left
          (width - center_x, height - center_y), // Bottom-right
        ];

        let projections: Vec<f32> = corners
          .iter()
          .map(|(dx, dy)| dx * cos_angle + dy * sin_angle)
          .collect();

        let max_proj = projections.iter().fold(0.0f32, |a, &b| a.max(b.abs()));
        max_proj.max(1.0) // Ensure we don't divide by zero
      };

      // Normalize projection to [0, 1] range
      (projection / max_projection + 1.0) / 2.0
    };

    let clamped_position = normalized_position.clamp(0.0, 1.0);

    // Find the two stops that bracket the current position
    let mut left_stop_index = 0;
    for (i, stop) in self.stops.iter().enumerate() {
      if stop.position <= clamped_position {
        left_stop_index = i;
      } else {
        break;
      }
    }

    // Handle edge cases
    if left_stop_index >= self.stops.len() - 1 {
      // We're past the last stop
      return self.stops[self.stops.len() - 1].color;
    }

    let left_stop = &self.stops[left_stop_index];
    let right_stop = &self.stops[left_stop_index + 1];

    // Handle the case where we're at or very close to an exact stop position
    // Use a practical epsilon for gradient positions (1/100th of the range)
    let position_epsilon = 0.01;

    // Check if we're close enough to the left stop
    if (clamped_position - left_stop.position).abs() < position_epsilon {
      return left_stop.color;
    }

    // Check if we're close enough to the right stop
    if (clamped_position - right_stop.position).abs() < position_epsilon {
      return right_stop.color;
    }

    // Special case: if we're at the first stop exactly
    if left_stop_index == 0 && clamped_position <= left_stop.position {
      return left_stop.color;
    }

    // Special case: if we're at the last stop exactly
    if left_stop_index >= self.stops.len() - 2 && clamped_position >= right_stop.position {
      return right_stop.color;
    }

    // Calculate interpolation factor based on actual positions
    let left_pos = left_stop.position;
    let right_pos = right_stop.position;
    let segment_length = right_pos - left_pos;

    let local_t = if segment_length <= 0.0 {
      0.0
    } else {
      ((clamped_position - left_pos) / segment_length).clamp(0.0, 1.0)
    };

    self.interpolate_colors(left_stop.color, right_stop.color, local_t)
  }
}

impl Gradient {
  fn interpolate_colors(&self, color1: Color, color2: Color, t: f32) -> Color {
    let (r1, g1, b1, a1) = color1.into();
    let (r2, g2, b2, a2) = color2.into();

    let r = (r1 as f32 * (1.0 - t) + r2 as f32 * t).round() as u8;
    let g = (g1 as f32 * (1.0 - t) + g2 as f32 * t).round() as u8;
    let b = (b1 as f32 * (1.0 - t) + b2 as f32 * t).round() as u8;
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
#[derive(Debug, Clone, Deserialize, Copy, Serialize, TS, PartialEq)]
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gradient_custom_positions_simple() {
    let gradient = Gradient {
      stops: vec![
        GradientStop {
          color: Color::Rgb(255, 0, 0), // red
          position: 0.0,
        },
        GradientStop {
          color: Color::Rgb(0, 255, 0), // green
          position: 0.5,
        },
        GradientStop {
          color: Color::Rgb(0, 0, 255), // blue
          position: 1.0,
        },
      ],
      angle: 0.0, // Horizontal gradient
    };

    // Test exact positions
    assert_eq!(gradient.at(100.0, 100.0, 0, 50), Color::Rgb(255, 0, 0));
    assert_eq!(gradient.at(100.0, 100.0, 99, 50), Color::Rgb(0, 0, 255));
  }

  #[test]
  fn test_gradient_non_uniform_positions() {
    let gradient = Gradient {
      stops: vec![
        GradientStop {
          color: Color::Rgb(255, 0, 0), // red
          position: 0.0,
        },
        GradientStop {
          color: Color::Rgb(0, 255, 0), // green
          position: 0.2,
        },
        GradientStop {
          color: Color::Rgb(0, 0, 255), // blue
          position: 0.8,
        },
      ],
      angle: 0.0,
    };

    // Test that positions are respected
    let color = gradient.at(100.0, 100.0, 0, 50);
    assert_eq!(color, Color::Rgb(255, 0, 0));

    let color = gradient.at(100.0, 100.0, 20, 50);
    assert_eq!(color, Color::Rgb(0, 255, 0));

    let color = gradient.at(100.0, 100.0, 80, 50);
    assert_eq!(color, Color::Rgb(0, 0, 255));
  }

  #[test]
  fn test_gradient_single_stop() {
    let gradient = Gradient {
      stops: vec![GradientStop {
        color: Color::Rgb(255, 128, 64),
        position: 0.5,
      }],
      angle: 45.0,
    };

    // Should always return the single color
    let color = gradient.at(100.0, 100.0, 0, 0);
    assert_eq!(color, Color::Rgb(255, 128, 64));

    let color = gradient.at(100.0, 100.0, 99, 99);
    assert_eq!(color, Color::Rgb(255, 128, 64));
  }

  #[test]
  fn test_gradient_empty_stops() {
    let gradient = Gradient {
      stops: vec![],
      angle: 90.0,
    };

    // Should return default color (black)
    let color = gradient.at(100.0, 100.0, 50, 50);
    assert_eq!(color, Color::Rgb(0, 0, 0));
  }

  #[test]
  fn test_gradient_with_transparency() {
    let gradient = Gradient {
      stops: vec![
        GradientStop {
          color: Color::Rgba(255, 0, 0, 0.5), // semi-transparent red
          position: 0.0,
        },
        GradientStop {
          color: Color::Rgba(0, 255, 0, 1.0), // opaque green
          position: 1.0,
        },
      ],
      angle: 0.0,
    };

    let color = gradient.at(100.0, 100.0, 50, 50);
    match color {
      Color::Rgba(_, _, _, a) => assert!((a - 0.75).abs() < 0.01),
      _ => panic!("Expected RGBA color with transparency"),
    }
  }

  #[test]
  fn test_gradient_interpolation_with_custom_positions() {
    let gradient = Gradient {
      stops: vec![
        GradientStop {
          color: Color::Rgb(255, 0, 0), // red
          position: 0.0,
        },
        GradientStop {
          color: Color::Rgb(0, 255, 0), // green
          position: 0.8,
        },
      ],
      angle: 0.0,
    };

    // Test interpolation between 0.0 and 0.8
    let color = gradient.at(100.0, 100.0, 40, 50); // 40% of width
    let (r, g, _b, _) = color.into();
    assert!(g > r); // More green than red (at 40/99 ≈ 40.4% of the way through a gradient from 0.0 to 0.8,
    // we're at t ≈ 0.505, which means more green than red)
    assert!(g > 0); // Some green component
  }
}
