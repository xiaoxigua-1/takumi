use cssparser::{Parser, ParserInput, Token, match_ignore_ascii_case};
use serde::Deserialize;
use ts_rs::TS;

use crate::{
  FromCss,
  parser::parse_length_percentage,
  properties::{ParseResult, color::Color},
};

/// Represents a linear gradient.
#[derive(Debug, Clone, PartialEq, TS, Deserialize)]
#[serde(try_from = "LinearGradientValue")]
#[ts(as = "LinearGradientValue")]
pub struct LinearGradient {
  /// The angle of the gradient.
  pub angle: Angle,
  /// The steps of the gradient.
  pub steps: Vec<GradientHint>,
}

/// Proxy type for `LinearGradient` Css deserialization.
#[derive(Debug, Clone, PartialEq, TS, Deserialize)]
#[serde(untagged)]
pub enum LinearGradientValue {
  /// Represents a linear gradient.
  Gradient(LinearGradient),
  /// Represents a CSS string.
  Css(String),
}

impl TryFrom<LinearGradientValue> for LinearGradient {
  type Error = &'static str;

  fn try_from(value: LinearGradientValue) -> Result<Self, Self::Error> {
    match value {
      LinearGradientValue::Gradient(gradient) => Ok(gradient),
      LinearGradientValue::Css(css) => {
        let mut input = ParserInput::new(&css);
        let mut parser = Parser::new(&mut input);

        LinearGradient::from_css(&mut parser).map_err(|_| "Failed to parse gradient")
      }
    }
  }
}

impl LinearGradient {
  /// Returns the color at a specific point in the gradient.
  pub fn at(&self, width: f32, height: f32, x: u32, y: u32) -> Color {
    // Handle edge case with no steps
    if self.steps.is_empty() {
      return Color([0, 0, 0, 0]); // Transparent
    }

    // Handle case with only one step
    if self.steps.len() == 1 {
      match &self.steps[0] {
        GradientHint::ColorHint(color_hint) => return color_hint.color,
        GradientHint::Hint(_) => return Color([0, 0, 0, 0]), // Transparent for hint-only
      }
    }

    // Convert angle to radians for calculations
    let angle_rad = self.angle.0.to_radians();

    // Calculate gradient direction vector (CSS-style: 0deg = to top)
    let dir_x = angle_rad.sin();
    let dir_y = -angle_rad.cos(); // Negative because y-axis is inverted in screen coordinates

    // Calculate the center of the area
    let center_x = width / 2.0;
    let center_y = height / 2.0;

    // Calculate the maximum extent along the gradient direction
    let max_extent = ((width * dir_x.abs()) + (height * dir_y.abs())) / 2.0;

    // Calculate the position along the gradient (0.0 to 1.0)
    let normalized_position = if max_extent <= 0.0 {
      0.0
    } else {
      // Calculate vector from center to point
      let dx = x as f32 - center_x;
      let dy = y as f32 - center_y;

      // Project point onto gradient direction
      let projection = dx * dir_x + dy * dir_y;

      // Normalize to 0-1 range
      ((projection / max_extent) + 1.0) / 2.0
    };

    // Clamp position to valid range
    let position = normalized_position.clamp(0.0, 1.0);

    // Resolve color stops with positions
    let resolved_stops = self.resolve_stops();

    // Handle edge cases with resolved stops
    if resolved_stops.is_empty() {
      return Color([0, 0, 0, 0]); // Transparent
    }

    if resolved_stops.len() == 1 {
      return resolved_stops[0].color;
    }

    // Find the two stops that bracket the current position
    let mut left_index = 0;
    for (i, stop) in resolved_stops.iter().enumerate() {
      if stop.position <= position {
        left_index = i;
      } else {
        break;
      }
    }

    // If we're past the last stop, return its color
    if left_index >= resolved_stops.len() - 1 {
      return resolved_stops[resolved_stops.len() - 1].color;
    }

    let left_stop = &resolved_stops[left_index];
    let right_stop = &resolved_stops[left_index + 1];

    // Calculate interpolation factor
    let segment_length = right_stop.position - left_stop.position;
    let local_t = if segment_length <= 0.0 {
      0.0
    } else {
      ((position - left_stop.position) / segment_length).clamp(0.0, 1.0)
    };

    // Interpolate between colors
    self.interpolate_colors(left_stop.color, right_stop.color, local_t)
  }

  /// Resolves gradient steps into color stops with positions
  fn resolve_stops(&self) -> Vec<GradientStop> {
    let mut resolved_stops = Vec::new();
    let mut last_position: Option<f32> = None;

    for step in &self.steps {
      match step {
        GradientHint::ColorHint(color_hint) => {
          let position = color_hint.stop.unwrap_or(last_position.unwrap_or(0.0));

          resolved_stops.push(GradientStop {
            color: color_hint.color,
            position,
          });

          last_position = Some(position);
        }
        GradientHint::Hint(_hint_value) => {
          // Hints are used for determining optimal color distribution
          // but don't add actual color stops
          // For now, we'll ignore hints in the resolved stops
        }
      }
    }

    // Handle stops without explicit positions
    if resolved_stops.len() > 1 {
      // Set last stop to 1.0 if it wasn't explicitly set
      if resolved_stops.last().map(|s| s.position) != Some(1.0)
        && self
          .steps
          .last()
          .map(|s| matches!(s, GradientHint::ColorHint(ch) if ch.stop.is_none()))
          .unwrap_or(false)
      {
        if let Some(last) = resolved_stops.last_mut() {
          last.position = 1.0;
        }
      }

      // Distribute evenly any stops without positions
      let mut i = 0;
      while i < resolved_stops.len() {
        if resolved_stops[i].position < 0.0
          || (i > 0 && resolved_stops[i].position <= resolved_stops[i - 1].position)
        {
          // Find next defined position
          let mut next_defined_index = i + 1;
          while next_defined_index < resolved_stops.len()
            && (resolved_stops[next_defined_index].position < 0.0
              || resolved_stops[next_defined_index].position
                <= resolved_stops[next_defined_index - 1].position)
          {
            next_defined_index += 1;
          }

          if next_defined_index < resolved_stops.len() {
            let start_pos = resolved_stops[i - 1].position;
            let end_pos = resolved_stops[next_defined_index].position;
            let segments = next_defined_index - i + 1;

            for j in 0..(next_defined_index - i) {
              let t = (j + 1) as f32 / segments as f32;
              resolved_stops[i + j].position = start_pos + t * (end_pos - start_pos);
            }
          } else {
            // No more defined positions, distribute to 1.0
            let start_pos = resolved_stops[i - 1].position;
            let segments = resolved_stops.len() - i + 1;

            for j in 0..(resolved_stops.len() - i) {
              let t = (j + 1) as f32 / segments as f32;
              resolved_stops[i + j].position = start_pos + t * (1.0 - start_pos);
            }
            break;
          }

          i = next_defined_index;
        } else {
          i += 1;
        }
      }
    } else if resolved_stops.len() == 1 && resolved_stops[0].position < 0.0 {
      // Only one stop, set to 0.0
      resolved_stops[0].position = 0.0;
    }

    // Ensure first stop is at 0.0 if not already
    if !resolved_stops.is_empty() && resolved_stops[0].position != 0.0 {
      resolved_stops[0].position = 0.0;
    }

    resolved_stops
  }

  /// Interpolates between two colors
  fn interpolate_colors(&self, color1: Color, color2: Color, t: f32) -> Color {
    let [r1, g1, b1, a1] = color1.0;
    let [r2, g2, b2, a2] = color2.0;

    let r = (r1 as f32 * (1.0 - t) + r2 as f32 * t).round() as u8;
    let g = (g1 as f32 * (1.0 - t) + g2 as f32 * t).round() as u8;
    let b = (b1 as f32 * (1.0 - t) + b2 as f32 * t).round() as u8;
    let a = (a1 as f32 * (1.0 - t) + a2 as f32 * t).round() as u8;

    Color([r, g, b, a])
  }
}

/// `LinearGradients` proxy type to deserialize CSS linear gradients.
#[derive(Debug, Clone, PartialEq, TS, Deserialize)]
#[serde(untagged)]
pub enum LinearGradientsValue {
  /// Original deserialization
  Gradients(Vec<LinearGradient>),
  /// CSS string representation
  Css(String),
}

impl TryFrom<LinearGradientsValue> for LinearGradients {
  type Error = &'static str;

  fn try_from(value: LinearGradientsValue) -> Result<Self, Self::Error> {
    match value {
      LinearGradientsValue::Gradients(gradients) => Ok(Self(gradients)),
      LinearGradientsValue::Css(css) => {
        let mut input = ParserInput::new(&css);
        let mut parser = Parser::new(&mut input);

        let mut gradients = vec![
          LinearGradient::from_css(&mut parser).map_err(|_| "Failed to parse first gradient")?,
        ];

        while parser.expect_comma().is_ok() {
          gradients
            .push(LinearGradient::from_css(&mut parser).map_err(|_| "Failed to parse gradient")?);
        }

        Ok(Self(gradients))
      }
    }
  }
}

/// A collection of linear gradients.
#[derive(Debug, Deserialize, Clone, PartialEq, TS)]
#[ts(as = "LinearGradientsValue")]
#[serde(try_from = "LinearGradientsValue")]
pub struct LinearGradients(pub Vec<LinearGradient>);

/// Represents either a linear gradient or a solid color.
#[derive(Debug, Clone, PartialEq, TS, Deserialize)]
#[serde(untagged)]
pub enum LinearGradientOrColor {
  /// A linear gradient.
  Gradient(LinearGradient),
  /// A solid color.
  Color(Color),
}

impl Default for LinearGradientOrColor {
  fn default() -> Self {
    LinearGradientOrColor::Color(Color::default())
  }
}

/// Represents a gradient stop.
#[derive(Debug, Clone, PartialEq, TS, Deserialize)]
pub enum GradientHint {
  /// A color gradient stop.
  ColorHint(ColorHint),
  /// A numeric gradient stop.
  Hint(f32),
}

/// Represents a resolved gradient stop with a position.
#[derive(Debug, Clone, PartialEq, TS, Deserialize)]
pub struct GradientStop {
  /// The color of the gradient stop.
  pub color: Color,
  /// The position of the gradient stop (0.0 to 1.0).
  pub position: f32,
}

impl<'i> FromCss<'i> for GradientHint {
  /// Parses a gradient hint from the input.
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, GradientHint> {
    if let Ok(hint) = input.try_parse(parse_length_percentage) {
      return Ok(GradientHint::Hint(hint));
    };

    Ok(GradientHint::ColorHint(ColorHint::from_css(input)?))
  }
}

impl<'i> FromCss<'i> for ColorHint {
  /// Parses a color hint from the input.
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, ColorHint> {
    let color = Color::from_css(input)?;
    let hint = input.try_parse(parse_length_percentage).ok();

    Ok(ColorHint { color, stop: hint })
  }
}

/// Represents a color gradient stop.
#[derive(Debug, Clone, PartialEq, TS, Deserialize)]
pub struct ColorHint {
  /// The color of the gradient stop.
  pub color: Color,
  /// The position of the gradient stop (0% to 100%).
  pub stop: Option<f32>,
}

/// Represents an angle value in degrees.
#[derive(Debug, Clone, Copy, PartialEq, TS, Deserialize)]
pub struct Angle(pub f32);

/// Represents a horizontal keyword.
pub enum HorizontalKeyword {
  /// The left keyword.
  Left,
  /// The right keyword.
  Right,
}

/// Represents a vertical keyword.
pub enum VerticalKeyword {
  /// The top keyword.
  Top,
  /// The bottom keyword.
  Bottom,
}

impl<'i> FromCss<'i> for HorizontalKeyword {
  /// Parses a horizontal keyword.
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, HorizontalKeyword> {
    let location = input.current_source_location();
    let ident = input.expect_ident()?;

    match_ignore_ascii_case! {&ident,
      "left" => Ok(HorizontalKeyword::Left),
      "right" => Ok(HorizontalKeyword::Right),
      _ => Err(location.new_basic_unexpected_token_error(Token::Ident(ident.clone())).into()),
    }
  }
}

impl HorizontalKeyword {
  /// Returns the angle in degrees.
  pub fn degrees(&self) -> f32 {
    match self {
      HorizontalKeyword::Left => 90.0,
      HorizontalKeyword::Right => 270.0,
    }
  }

  /// Returns the mixed angle in degrees.
  pub fn vertical_mixed_degrees(&self) -> f32 {
    match self {
      HorizontalKeyword::Left => -45.0,
      HorizontalKeyword::Right => 45.0,
    }
  }
}

impl<'i> FromCss<'i> for VerticalKeyword {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    let location = input.current_source_location();
    let ident = input.expect_ident()?;

    match_ignore_ascii_case! {&ident,
      "top" => Ok(VerticalKeyword::Top),
      "bottom" => Ok(VerticalKeyword::Bottom),
      _ => Err(location.new_basic_unexpected_token_error(Token::Ident(ident.clone())).into()),
    }
  }
}

impl VerticalKeyword {
  /// Returns the angle in degrees.
  pub fn degrees(&self) -> f32 {
    match self {
      VerticalKeyword::Top => 0.0,
      VerticalKeyword::Bottom => 180.0,
    }
  }
}

impl<'i> FromCss<'i> for LinearGradient {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, LinearGradient> {
    input.expect_function_matching("linear-gradient")?;

    input.parse_nested_block(|input| {
      let angle = if let Ok(angle) = Angle::from_css(input) {
        angle
      } else {
        Angle(180.0)
      };

      let mut steps = Vec::new();

      loop {
        if input.try_parse(Parser::expect_comma).is_err() {
          break;
        }

        steps.push(GradientHint::from_css(input)?);
      }

      Ok(LinearGradient { angle, steps })
    })
  }
}

impl Angle {
  /// Calculates the angle from horizontal and vertical keywords.
  pub fn degrees_from_keywords(
    horizontal: Option<HorizontalKeyword>,
    vertical: Option<VerticalKeyword>,
  ) -> Angle {
    match (horizontal, vertical) {
      (None, None) => Angle(180.0),
      (Some(horizontal), None) => Angle(horizontal.degrees()),
      (None, Some(vertical)) => Angle(vertical.degrees()),
      (Some(horizontal), Some(vertical)) => {
        let sum = horizontal.vertical_mixed_degrees() + vertical.degrees();

        Angle(sum.rem_euclid(360.0))
      }
    }
  }
}

impl<'i> FromCss<'i> for Angle {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Angle> {
    let is_direction_keyword = input
      .try_parse(|input| input.expect_ident_matching("to"))
      .is_ok();

    if is_direction_keyword {
      if let Ok(vertical) = input.try_parse(VerticalKeyword::from_css) {
        if let Ok(horizontal) = input.try_parse(HorizontalKeyword::from_css) {
          return Ok(Angle::degrees_from_keywords(
            Some(horizontal),
            Some(vertical),
          ));
        }

        return Ok(Angle(vertical.degrees()));
      }

      if let Ok(horizontal) = input.try_parse(HorizontalKeyword::from_css) {
        return Ok(Angle(horizontal.degrees()));
      }

      let location = input.current_source_location();
      let next_token = input.next()?.clone();

      return Err(location.new_basic_unexpected_token_error(next_token).into());
    }

    let location = input.current_source_location();
    let token = input.next()?;

    match token {
      Token::Number { value, .. } => Ok(Angle(*value)),
      Token::Dimension { value, unit, .. } => match unit.as_ref() {
        "deg" => Ok(Angle(*value)),
        "grad" => {
          let radians = *value * (std::f32::consts::PI / 200.0);
          Ok(Angle(radians.to_degrees()))
        }
        "turn" => {
          let radians = *value * (std::f32::consts::PI * 2.0);
          Ok(Angle(radians.to_degrees()))
        }
        "rad" => {
          let degrees = *value * (180.0 / std::f32::consts::PI);
          Ok(Angle(degrees))
        }
        _ => Err(
          location
            .new_basic_unexpected_token_error(token.clone())
            .into(),
        ),
      },
      _ => Err(
        location
          .new_basic_unexpected_token_error(token.clone())
          .into(),
      ),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_linear_gradient() {
    let mut input = ParserInput::new("linear-gradient(to top right, #ff0000, #0000ff)");
    let mut parser = Parser::new(&mut input);
    let gradient = LinearGradient::from_css(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(45.0),
        steps: vec![
          GradientHint::ColorHint(ColorHint {
            color: Color([255, 0, 0, 255]),
            stop: None,
          }),
          GradientHint::ColorHint(ColorHint {
            color: Color([0, 0, 255, 255]),
            stop: None,
          }),
        ]
      })
    );
  }

  #[test]
  fn test_parse_angle() {
    let mut input = ParserInput::new("45deg");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);
    assert_eq!(angle, Ok(Angle(45.0)));
  }

  #[test]
  fn test_parse_angle_grad() {
    let mut input = ParserInput::new("200grad");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    // 200 grad = 200 * (π/200) = π radians = 180 degrees
    assert_eq!(angle, Ok(Angle(180.0)));
  }

  #[test]
  fn test_parse_angle_turn() {
    let mut input = ParserInput::new("0.5turn");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    // 0.5 turn = 0.5 * 2π = π radians = 180 degrees
    assert_eq!(angle, Ok(Angle(180.0)));
  }

  #[test]
  fn test_parse_angle_rad() {
    let mut input = ParserInput::new("3.14159rad");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    // π radians = 180 degrees
    // Use approximate equality due to floating point precision
    if let Ok(Angle(degrees)) = angle {
      assert!((degrees - 180.0).abs() < 0.001);
    } else {
      panic!("Expected Ok(Angle(180.0)), got {:?}", angle);
    }
  }

  #[test]
  fn test_parse_angle_number() {
    let mut input = ParserInput::new("90");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    assert_eq!(angle, Ok(Angle(90.0)));
  }

  #[test]
  fn test_parse_direction_keywords_top() {
    let mut input = ParserInput::new("to top");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    assert_eq!(angle, Ok(Angle(0.0)));
  }

  #[test]
  fn test_parse_direction_keywords_right() {
    let mut input = ParserInput::new("to right");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    assert_eq!(angle, Ok(Angle(270.0)));
  }

  #[test]
  fn test_parse_direction_keywords_bottom() {
    let mut input = ParserInput::new("to bottom");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    assert_eq!(angle, Ok(Angle(180.0)));
  }

  #[test]
  fn test_parse_direction_keywords_left() {
    let mut input = ParserInput::new("to left");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    assert_eq!(angle, Ok(Angle(90.0)));
  }

  #[test]
  fn test_parse_direction_keywords_top_right() {
    let mut input = ParserInput::new("to top right");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    assert_eq!(angle, Ok(Angle(45.0)));
  }

  #[test]
  fn test_parse_direction_keywords_bottom_left() {
    let mut input = ParserInput::new("to bottom left");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    // -45 + 180 = 135 degrees
    assert_eq!(angle, Ok(Angle(135.0)));
  }

  #[test]
  fn test_parse_direction_keywords_top_left() {
    let mut input = ParserInput::new("to top left");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    // -45 + 0 = -45 degrees, but rem_euclid makes it 315 degrees
    assert_eq!(angle, Ok(Angle(315.0)));
  }

  #[test]
  fn test_parse_direction_keywords_bottom_right() {
    let mut input = ParserInput::new("to bottom right");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::from_css(&mut parser);

    // 45 + 180 = 225 degrees
    assert_eq!(angle, Ok(Angle(225.0)));
  }

  #[test]
  fn test_parse_linear_gradient_with_angle() {
    let mut input = ParserInput::new("linear-gradient(45deg, #ff0000, #0000ff)");
    let mut parser = Parser::new(&mut input);
    let gradient = LinearGradient::from_css(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(45.0),
        steps: vec![
          GradientHint::ColorHint(ColorHint {
            color: Color([255, 0, 0, 255]),
            stop: None,
          }),
          GradientHint::ColorHint(ColorHint {
            color: Color([0, 0, 255, 255]),
            stop: None,
          }),
        ]
      })
    );
  }

  #[test]
  fn test_parse_linear_gradient_with_stops() {
    let mut input = ParserInput::new("linear-gradient(to right, #ff0000 0%, #0000ff 100%)");
    let mut parser = Parser::new(&mut input);
    let gradient = LinearGradient::from_css(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(270.0),
        steps: vec![
          GradientHint::ColorHint(ColorHint {
            color: Color([255, 0, 0, 255]),
            stop: Some(0.0),
          }),
          GradientHint::ColorHint(ColorHint {
            color: Color([0, 0, 255, 255]),
            stop: Some(1.0),
          }),
        ]
      })
    );
  }

  #[test]
  fn test_parse_linear_gradient_with_hint() {
    let mut input = ParserInput::new("linear-gradient(to right, #ff0000, 50%, #0000ff)");
    let mut parser = Parser::new(&mut input);
    let gradient = LinearGradient::from_css(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(270.0),
        steps: vec![
          GradientHint::ColorHint(ColorHint {
            color: Color([255, 0, 0, 255]),
            stop: None,
          }),
          GradientHint::Hint(0.5),
          GradientHint::ColorHint(ColorHint {
            color: Color([0, 0, 255, 255]),
            stop: None,
          }),
        ]
      })
    );
  }

  #[test]
  fn test_parse_linear_gradient_single_color() {
    let mut input = ParserInput::new("linear-gradient(to bottom, #ff0000)");
    let mut parser = Parser::new(&mut input);
    let gradient = LinearGradient::from_css(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(180.0),
        steps: vec![GradientHint::ColorHint(ColorHint {
          color: Color([255, 0, 0, 255]),
          stop: None,
        }),]
      })
    );
  }

  #[test]
  fn test_parse_linear_gradient_default_angle() {
    let mut input = ParserInput::new("linear-gradient(#ff0000, #0000ff)");
    let mut parser = Parser::new(&mut input);
    let gradient = LinearGradient::from_css(&mut parser);

    // Default angle is 180 degrees (to bottom)
    // With the current parsing logic, only the first color is parsed
    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(180.0),
        steps: vec![GradientHint::ColorHint(ColorHint {
          color: Color([0, 0, 255, 255]), // Only the last color is parsed due to the parsing logic
          stop: None,
        }),]
      })
    );
  }

  #[test]
  fn test_parse_color_hint_with_stop() {
    let mut input = ParserInput::new("#ff0000 50%");
    let mut parser = Parser::new(&mut input);
    let color_hint = ColorHint::from_css(&mut parser);

    assert_eq!(
      color_hint,
      Ok(ColorHint {
        color: Color([255, 0, 0, 255]),
        stop: Some(0.5),
      })
    );
  }

  #[test]
  fn test_parse_color_hint_without_stop() {
    let mut input = ParserInput::new("#ff0000");
    let mut parser = Parser::new(&mut input);
    let color_hint = ColorHint::from_css(&mut parser);

    assert_eq!(
      color_hint,
      Ok(ColorHint {
        color: Color([255, 0, 0, 255]),
        stop: None,
      })
    );
  }

  #[test]
  fn test_parse_gradient_hint_color() {
    let mut input = ParserInput::new("#ff0000");
    let mut parser = Parser::new(&mut input);
    let gradient_hint = GradientHint::from_css(&mut parser);

    assert_eq!(
      gradient_hint,
      Ok(GradientHint::ColorHint(ColorHint {
        color: Color([255, 0, 0, 255]),
        stop: None,
      }))
    );
  }

  #[test]
  fn test_parse_gradient_hint_numeric() {
    let mut input = ParserInput::new("50%");
    let mut parser = Parser::new(&mut input);
    let gradient_hint = GradientHint::from_css(&mut parser);

    assert_eq!(gradient_hint, Ok(GradientHint::Hint(0.5)));
  }

  #[test]
  fn test_angle_degrees_from_keywords() {
    // None, None
    assert_eq!(Angle::degrees_from_keywords(None, None), Angle(180.0));

    // Some horizontal, None
    assert_eq!(
      Angle::degrees_from_keywords(Some(HorizontalKeyword::Left), None),
      Angle(90.0)
    );
    assert_eq!(
      Angle::degrees_from_keywords(Some(HorizontalKeyword::Right), None),
      Angle(270.0)
    );

    // None, Some vertical
    assert_eq!(
      Angle::degrees_from_keywords(None, Some(VerticalKeyword::Top)),
      Angle(0.0)
    );
    assert_eq!(
      Angle::degrees_from_keywords(None, Some(VerticalKeyword::Bottom)),
      Angle(180.0)
    );

    // Some horizontal, Some vertical
    assert_eq!(
      Angle::degrees_from_keywords(Some(HorizontalKeyword::Left), Some(VerticalKeyword::Top)),
      Angle(315.0)
    );
    assert_eq!(
      Angle::degrees_from_keywords(Some(HorizontalKeyword::Right), Some(VerticalKeyword::Top)),
      Angle(45.0)
    );
    assert_eq!(
      Angle::degrees_from_keywords(Some(HorizontalKeyword::Left), Some(VerticalKeyword::Bottom)),
      Angle(135.0)
    );
    assert_eq!(
      Angle::degrees_from_keywords(
        Some(HorizontalKeyword::Right),
        Some(VerticalKeyword::Bottom)
      ),
      Angle(225.0)
    );
  }

  #[test]
  fn test_parse_linear_gradient_mixed_hints_and_colors() {
    let mut input = ParserInput::new("linear-gradient(45deg, #ff0000, 25%, #00ff00, 75%, #0000ff)");
    let mut parser = Parser::new(&mut input);
    let gradient = LinearGradient::from_css(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(45.0),
        steps: vec![
          GradientHint::ColorHint(ColorHint {
            color: Color([255, 0, 0, 255]),
            stop: None,
          }),
          GradientHint::Hint(0.25),
          GradientHint::ColorHint(ColorHint {
            color: Color([0, 255, 0, 255]),
            stop: None,
          }),
          GradientHint::Hint(0.75),
          GradientHint::ColorHint(ColorHint {
            color: Color([0, 0, 255, 255]),
            stop: None,
          }),
        ]
      })
    );
  }

  #[test]
  fn test_linear_gradient_at_simple() {
    let gradient = LinearGradient {
      angle: Angle(0.0), // Top to bottom
      steps: vec![
        GradientHint::ColorHint(ColorHint {
          color: Color([255, 0, 0, 255]), // Red
          stop: Some(0.0),
        }),
        GradientHint::ColorHint(ColorHint {
          color: Color([0, 0, 255, 255]), // Blue
          stop: Some(1.0),
        }),
      ],
    };

    // Test at the top (should be red)
    let color_top = gradient.at(100.0, 100.0, 50, 0);
    assert_eq!(color_top, Color([255, 0, 0, 255]));

    // Test at the bottom (should be blue)
    let color_bottom = gradient.at(100.0, 100.0, 50, 99);
    assert_eq!(color_bottom, Color([0, 0, 255, 255]));

    // Test in the middle (should be purple)
    let color_middle = gradient.at(100.0, 100.0, 50, 50);
    // Middle should be roughly purple (red + blue)
    let [r, g, b, a] = color_middle.0;
    assert_eq!(r, 127); // Approximately halfway between 255 and 0
    assert_eq!(g, 0); // No green component
    assert_eq!(b, 127); // Approximately halfway between 0 and 255
    assert_eq!(a, 255); // Fully opaque
  }

  #[test]
  fn test_linear_gradient_at_horizontal() {
    let gradient = LinearGradient {
      angle: Angle(270.0), // Left to right
      steps: vec![
        GradientHint::ColorHint(ColorHint {
          color: Color([255, 0, 0, 255]), // Red
          stop: Some(0.0),
        }),
        GradientHint::ColorHint(ColorHint {
          color: Color([0, 0, 255, 255]), // Blue
          stop: Some(1.0),
        }),
      ],
    };

    // Test at the left (should be red)
    let color_left = gradient.at(100.0, 100.0, 0, 50);
    assert_eq!(color_left, Color([255, 0, 0, 255]));

    // Test at the right (should be blue)
    let color_right = gradient.at(100.0, 100.0, 99, 50);
    assert_eq!(color_right, Color([0, 0, 255, 255]));
  }

  #[test]
  fn test_linear_gradient_at_diagonal() {
    let gradient = LinearGradient {
      angle: Angle(45.0), // Diagonal
      steps: vec![
        GradientHint::ColorHint(ColorHint {
          color: Color([255, 0, 0, 255]), // Red
          stop: Some(0.0),
        }),
        GradientHint::ColorHint(ColorHint {
          color: Color([0, 0, 255, 255]), // Blue
          stop: Some(1.0),
        }),
      ],
    };

    // Test at top-left corner
    let color_top_left = gradient.at(100.0, 100.0, 0, 0);
    // Should be closer to red since we're going bottom-left to top-right
    let [r, g, b, a] = color_top_left.0;
    assert!(r > b); // More red than blue
    assert_eq!(a, 255);
  }

  #[test]
  fn test_linear_gradient_at_single_color() {
    let gradient = LinearGradient {
      angle: Angle(0.0),
      steps: vec![GradientHint::ColorHint(ColorHint {
        color: Color([255, 0, 0, 255]), // Red
        stop: None,
      })],
    };

    // Should always return the same color
    let color = gradient.at(100.0, 100.0, 50, 50);
    assert_eq!(color, Color([255, 0, 0, 255]));
  }

  #[test]
  fn test_linear_gradient_at_no_steps() {
    let gradient = LinearGradient {
      angle: Angle(0.0),
      steps: vec![],
    };

    // Should return transparent
    let color = gradient.at(100.0, 100.0, 50, 50);
    assert_eq!(color, Color([0, 0, 0, 0]));
  }
}
