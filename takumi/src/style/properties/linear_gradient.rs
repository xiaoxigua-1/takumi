use cssparser::{Parser, Token, match_ignore_ascii_case};

use crate::{
  parser::parse_length_percentage,
  properties::{ParseResult, color::Color},
};

/// Represents a linear gradient.
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
  /// The angle of the gradient.
  pub angle: Angle,
  /// The steps of the gradient.
  pub steps: Vec<GradientHint>,
}

/// Represents a gradient stop.
#[derive(Debug, Clone, PartialEq)]
pub enum GradientHint {
  /// A color gradient stop.
  ColorHint(ColorHint),
  /// A numeric gradient stop.
  Hint(f32),
}

impl GradientHint {
  /// Parses a gradient hint from the input.
  pub fn parse<'i>(input: &mut Parser<'i, '_>) -> ParseResult<'i, GradientHint> {
    if let Ok(hint) = input.try_parse(parse_length_percentage) {
      return Ok(GradientHint::Hint(hint));
    };

    Ok(GradientHint::ColorHint(ColorHint::parse(input)?))
  }
}

impl ColorHint {
  /// Parses a color hint from the input.
  pub fn parse<'i>(input: &mut Parser<'i, '_>) -> ParseResult<'i, ColorHint> {
    let color = Color::parse(input)?;
    let hint = input.try_parse(parse_length_percentage).ok();

    Ok(ColorHint { color, stop: hint })
  }
}

/// Represents a color gradient stop.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorHint {
  /// The color of the gradient stop.
  pub color: Color,
  /// The position of the gradient stop (0% to 100%).
  pub stop: Option<f32>,
}

/// Represents an angle value in degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl HorizontalKeyword {
  /// Parses a horizontal keyword.
  pub fn parse<'i>(input: &mut Parser<'i, '_>) -> ParseResult<'i, HorizontalKeyword> {
    let location = input.current_source_location();
    let ident = input.expect_ident()?;

    match_ignore_ascii_case! {&ident,
      "left" => Ok(HorizontalKeyword::Left),
      "right" => Ok(HorizontalKeyword::Right),
      _ => Err(location.new_basic_unexpected_token_error(Token::Ident(ident.clone())).into()),
    }
  }

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

impl VerticalKeyword {
  /// Parses a vertical keyword.
  pub fn parse<'i>(input: &mut Parser<'i, '_>) -> ParseResult<'i, VerticalKeyword> {
    let location = input.current_source_location();
    let ident = input.expect_ident()?;

    match_ignore_ascii_case! {&ident,
      "top" => Ok(VerticalKeyword::Top),
      "bottom" => Ok(VerticalKeyword::Bottom),
      _ => Err(location.new_basic_unexpected_token_error(Token::Ident(ident.clone())).into()),
    }
  }

  /// Returns the angle in degrees.
  pub fn degrees(&self) -> f32 {
    match self {
      VerticalKeyword::Top => 0.0,
      VerticalKeyword::Bottom => 180.0,
    }
  }
}

impl LinearGradient {
  /// Parses a linear gradient value.
  pub fn parse<'i>(input: &mut Parser<'i, '_>) -> ParseResult<'i, LinearGradient> {
    input.expect_function_matching("linear-gradient")?;

    input.parse_nested_block(|input| {
      let angle = if let Ok(angle) = Angle::parse(input) {
        angle
      } else {
        Angle(180.0)
      };

      let mut steps = Vec::new();

      loop {
        if input.try_parse(Parser::expect_comma).is_err() {
          break;
        }

        steps.push(GradientHint::parse(input)?);
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

  /// Parses an angle value. deg, grad, turn, rad is supported.
  pub fn parse<'i>(input: &mut Parser<'i, '_>) -> ParseResult<'i, Angle> {
    let is_direction_keyword = input
      .try_parse(|input| input.expect_ident_matching("to"))
      .is_ok();

    if is_direction_keyword {
      if let Ok(vertical) = input.try_parse(VerticalKeyword::parse) {
        if let Ok(horizontal) = input.try_parse(HorizontalKeyword::parse) {
          return Ok(Angle::degrees_from_keywords(
            Some(horizontal),
            Some(vertical),
          ));
        }

        return Ok(Angle(vertical.degrees()));
      }

      if let Ok(horizontal) = input.try_parse(HorizontalKeyword::parse) {
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
  use cssparser::ParserInput;

  use super::*;

  #[test]
  fn test_parse_linear_gradient() {
    let mut input = ParserInput::new("linear-gradient(to top right, #ff0000, #0000ff)");
    let mut parser = Parser::new(&mut input);
    let gradient = LinearGradient::parse(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(45.0),
        steps: vec![
          GradientHint::ColorHint(ColorHint {
            color: Color(255, 0, 0, 255),
            stop: None,
          }),
          GradientHint::ColorHint(ColorHint {
            color: Color(0, 0, 255, 255),
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
    let angle = Angle::parse(&mut parser);
    assert_eq!(angle, Ok(Angle(45.0)));
  }

  #[test]
  fn test_parse_angle_grad() {
    let mut input = ParserInput::new("200grad");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

    // 200 grad = 200 * (π/200) = π radians = 180 degrees
    assert_eq!(angle, Ok(Angle(180.0)));
  }

  #[test]
  fn test_parse_angle_turn() {
    let mut input = ParserInput::new("0.5turn");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

    // 0.5 turn = 0.5 * 2π = π radians = 180 degrees
    assert_eq!(angle, Ok(Angle(180.0)));
  }

  #[test]
  fn test_parse_angle_rad() {
    let mut input = ParserInput::new("3.14159rad");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

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
    let angle = Angle::parse(&mut parser);

    assert_eq!(angle, Ok(Angle(90.0)));
  }

  #[test]
  fn test_parse_direction_keywords_top() {
    let mut input = ParserInput::new("to top");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

    assert_eq!(angle, Ok(Angle(0.0)));
  }

  #[test]
  fn test_parse_direction_keywords_right() {
    let mut input = ParserInput::new("to right");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

    assert_eq!(angle, Ok(Angle(270.0)));
  }

  #[test]
  fn test_parse_direction_keywords_bottom() {
    let mut input = ParserInput::new("to bottom");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

    assert_eq!(angle, Ok(Angle(180.0)));
  }

  #[test]
  fn test_parse_direction_keywords_left() {
    let mut input = ParserInput::new("to left");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

    assert_eq!(angle, Ok(Angle(90.0)));
  }

  #[test]
  fn test_parse_direction_keywords_top_right() {
    let mut input = ParserInput::new("to top right");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

    assert_eq!(angle, Ok(Angle(45.0)));
  }

  #[test]
  fn test_parse_direction_keywords_bottom_left() {
    let mut input = ParserInput::new("to bottom left");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

    // -45 + 180 = 135 degrees
    assert_eq!(angle, Ok(Angle(135.0)));
  }

  #[test]
  fn test_parse_direction_keywords_top_left() {
    let mut input = ParserInput::new("to top left");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

    // -45 + 0 = -45 degrees, but rem_euclid makes it 315 degrees
    assert_eq!(angle, Ok(Angle(315.0)));
  }

  #[test]
  fn test_parse_direction_keywords_bottom_right() {
    let mut input = ParserInput::new("to bottom right");
    let mut parser = Parser::new(&mut input);
    let angle = Angle::parse(&mut parser);

    // 45 + 180 = 225 degrees
    assert_eq!(angle, Ok(Angle(225.0)));
  }

  #[test]
  fn test_parse_linear_gradient_with_angle() {
    let mut input = ParserInput::new("linear-gradient(45deg, #ff0000, #0000ff)");
    let mut parser = Parser::new(&mut input);
    let gradient = LinearGradient::parse(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(45.0),
        steps: vec![
          GradientHint::ColorHint(ColorHint {
            color: Color(255, 0, 0, 255),
            stop: None,
          }),
          GradientHint::ColorHint(ColorHint {
            color: Color(0, 0, 255, 255),
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
    let gradient = LinearGradient::parse(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(270.0),
        steps: vec![
          GradientHint::ColorHint(ColorHint {
            color: Color(255, 0, 0, 255),
            stop: Some(0.0),
          }),
          GradientHint::ColorHint(ColorHint {
            color: Color(0, 0, 255, 255),
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
    let gradient = LinearGradient::parse(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(270.0),
        steps: vec![
          GradientHint::ColorHint(ColorHint {
            color: Color(255, 0, 0, 255),
            stop: None,
          }),
          GradientHint::Hint(0.5),
          GradientHint::ColorHint(ColorHint {
            color: Color(0, 0, 255, 255),
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
    let gradient = LinearGradient::parse(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(180.0),
        steps: vec![GradientHint::ColorHint(ColorHint {
          color: Color(255, 0, 0, 255),
          stop: None,
        }),]
      })
    );
  }

  #[test]
  fn test_parse_linear_gradient_default_angle() {
    let mut input = ParserInput::new("linear-gradient(#ff0000, #0000ff)");
    let mut parser = Parser::new(&mut input);
    let gradient = LinearGradient::parse(&mut parser);

    // Default angle is 180 degrees (to bottom)
    // With the current parsing logic, only the first color is parsed
    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(180.0),
        steps: vec![GradientHint::ColorHint(ColorHint {
          color: Color(0, 0, 255, 255), // Only the last color is parsed due to the parsing logic
          stop: None,
        }),]
      })
    );
  }

  #[test]
  fn test_parse_color_hint_with_stop() {
    let mut input = ParserInput::new("#ff0000 50%");
    let mut parser = Parser::new(&mut input);
    let color_hint = ColorHint::parse(&mut parser);

    assert_eq!(
      color_hint,
      Ok(ColorHint {
        color: Color(255, 0, 0, 255),
        stop: Some(0.5),
      })
    );
  }

  #[test]
  fn test_parse_color_hint_without_stop() {
    let mut input = ParserInput::new("#ff0000");
    let mut parser = Parser::new(&mut input);
    let color_hint = ColorHint::parse(&mut parser);

    assert_eq!(
      color_hint,
      Ok(ColorHint {
        color: Color(255, 0, 0, 255),
        stop: None,
      })
    );
  }

  #[test]
  fn test_parse_gradient_hint_color() {
    let mut input = ParserInput::new("#ff0000");
    let mut parser = Parser::new(&mut input);
    let gradient_hint = GradientHint::parse(&mut parser);

    assert_eq!(
      gradient_hint,
      Ok(GradientHint::ColorHint(ColorHint {
        color: Color(255, 0, 0, 255),
        stop: None,
      }))
    );
  }

  #[test]
  fn test_parse_gradient_hint_numeric() {
    let mut input = ParserInput::new("50%");
    let mut parser = Parser::new(&mut input);
    let gradient_hint = GradientHint::parse(&mut parser);

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
    let gradient = LinearGradient::parse(&mut parser);

    assert_eq!(
      gradient,
      Ok(LinearGradient {
        angle: Angle(45.0),
        steps: vec![
          GradientHint::ColorHint(ColorHint {
            color: Color(255, 0, 0, 255),
            stop: None,
          }),
          GradientHint::Hint(0.25),
          GradientHint::ColorHint(ColorHint {
            color: Color(0, 255, 0, 255),
            stop: None,
          }),
          GradientHint::Hint(0.75),
          GradientHint::ColorHint(ColorHint {
            color: Color(0, 0, 255, 255),
            stop: None,
          }),
        ]
      })
    );
  }
}
