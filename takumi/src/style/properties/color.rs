use csscolorparser::{NAMED_COLORS, ParseColorError};
use cssparser::{Parser, ToCss, Token};

use crate::properties::ParseResult;

/// Represents a RGBA color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
  /// Parses a color value.
  pub fn parse<'i>(input: &mut Parser<'i, '_>) -> ParseResult<'i, Color> {
    let location = input.current_source_location();
    let position = input.position();
    let token = input.next()?;

    match *token {
      Token::Hash(_) | Token::IDHash(_) => {
        parse_color_string(&token.to_css_string()).map_err(|_| {
          location
            .new_basic_unexpected_token_error(token.clone())
            .into()
        })
      }
      Token::Ident(ref ident) => {
        if ident.as_ref() == "transparent" {
          return Ok(Color(0, 0, 0, 0));
        }

        let Some([r, g, b]) = NAMED_COLORS.get(ident) else {
          return Err(
            location
              .new_basic_unexpected_token_error(token.clone())
              .into(),
          );
        };

        Ok(Color(*r, *g, *b, 255))
      }
      Token::Function(_) => {
        // Have to clone to persist token, and allow input to be borrowed
        let token = token.clone();

        input.parse_nested_block(|input| {
          while input.next().is_ok() {}

          // Slice from the function name till before the closing parenthesis
          let body = input.slice_from(position);

          let mut function = body.to_string();

          // Add closing parenthesis
          function.push(')');

          parse_color_string(&function)
            .map_err(|_| location.new_basic_unexpected_token_error(token).into())
        })
      }
      _ => Err(
        location
          .new_basic_unexpected_token_error(token.clone())
          .into(),
      ),
    }
  }
}

fn parse_color_string(string: &str) -> Result<Color, ParseColorError> {
  csscolorparser::parse(string).map(|color| {
    let [r, g, b, a] = color.to_rgba8();

    Color(r, g, b, a)
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use cssparser::{Parser, ParserInput};

  fn parse_color_str(input: &str) -> ParseResult<'_, Color> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);

    Color::parse(&mut parser)
  }

  #[test]
  fn test_parse_hex_color_3_digits() {
    // Test 3-digit hex color
    let result = parse_color_str("#f09").unwrap();
    assert_eq!(result, Color(255, 0, 153, 255));
  }

  #[test]
  fn test_parse_hex_color_6_digits() {
    // Test 6-digit hex color
    let result = parse_color_str("#ff0099").unwrap();
    assert_eq!(result, Color(255, 0, 153, 255));
  }

  #[test]
  fn test_parse_color_transparent() {
    // Test parsing transparent keyword
    let result = parse_color_str("transparent").unwrap();
    assert_eq!(result, Color(0, 0, 0, 0));
  }

  #[test]
  fn test_parse_color_rgb_function() {
    // Test parsing rgb() function through main parse function
    let result = parse_color_str("rgb(255, 0, 153)").unwrap();
    assert_eq!(result, Color(255, 0, 153, 255));
  }

  #[test]
  fn test_parse_color_rgba_function() {
    // Test parsing rgba() function through main parse function
    let result = parse_color_str("rgba(255, 0, 153, 0.5)").unwrap();
    assert_eq!(result, Color(255, 0, 153, 128));
  }

  #[test]
  fn test_parse_color_rgb_space_separated() {
    // Test parsing rgb() function with space-separated values
    let result = parse_color_str("rgb(255 0 153)").unwrap();
    assert_eq!(result, Color(255, 0, 153, 255));
  }

  #[test]
  fn test_parse_color_rgb_with_alpha_slash() {
    // Test parsing rgb() function with alpha value using slash
    let result = parse_color_str("rgb(255 0 153 / 0.5)").unwrap();
    assert_eq!(result, Color(255, 0, 153, 128));
  }

  #[test]
  fn test_parse_color_invalid_function() {
    // Test parsing invalid function
    let result = parse_color_str("invalid(255, 0, 153)");
    assert!(result.is_err());
  }
}
