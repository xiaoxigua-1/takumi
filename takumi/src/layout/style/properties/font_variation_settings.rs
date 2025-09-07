use cssparser::{Parser, Token};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::layout::style::properties::{FromCss, ParseResult};

/// Represents a single font variation setting with an axis name and value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct FontVariationSetting {
  /// The four-character axis name (e.g., "wght", "wdth", "ital")
  pub axis: String,
  /// The numeric value for this axis
  pub value: f32,
}

/// Controls variable font axis values via CSS font-variation-settings property.
///
/// This allows fine-grained control over variable font characteristics like weight,
/// width, slant, and other custom axes defined in the font.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
pub struct FontVariationSettings(pub Vec<FontVariationSetting>);

impl<'i> FromCss<'i> for FontVariationSettings {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    let mut settings = Vec::new();

    loop {
      // Parse axis name (string or identifier)
      let location = input.current_source_location();
      let axis = match input.next()? {
        Token::QuotedString(s) => s.to_string(),
        Token::Ident(s) => s.to_string(),
        other => {
          return Err(
            location
              .new_basic_unexpected_token_error(other.clone())
              .into(),
          );
        }
      };

      // Validate axis name (should be 4 characters for registered axes, but custom can be any)
      if axis.is_empty() {
        let location = input.current_source_location();
        return Err(
          location
            .new_basic_unexpected_token_error(Token::Ident("".into()))
            .into(),
        );
      }

      // Parse value
      let value = input.expect_number()?;

      settings.push(FontVariationSetting {
        axis: axis.to_string(),
        value,
      });

      // Check for comma (more settings) or end
      if input.try_parse(Parser::expect_comma).is_err() {
        break;
      }
    }

    Ok(FontVariationSettings(settings))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cssparser::ParserInput;

  #[test]
  fn test_font_variation_settings_parsing() {
    let test_cases = vec![
      ("\"wght\" 400", vec![("wght", 400.0)]),
      (
        "\"wdth\" 100, \"wght\" 700",
        vec![("wdth", 100.0), ("wght", 700.0)],
      ),
      ("wght 400, wdth 125", vec![("wght", 400.0), ("wdth", 125.0)]),
      (
        "\"ital\" 0.5, \"slnt\" -10",
        vec![("ital", 0.5), ("slnt", -10.0)],
      ),
    ];

    for (input, expected) in test_cases {
      let mut input = ParserInput::new(input);
      let mut parser = Parser::new(&mut input);
      let result = FontVariationSettings::from_css(&mut parser).unwrap();

      assert_eq!(result.0.len(), expected.len());
      for (i, (axis, value)) in expected.iter().enumerate() {
        assert_eq!(result.0[i].axis, *axis);
        assert_eq!(result.0[i].value, *value);
      }
    }
  }

  #[test]
  fn test_font_variation_settings_empty() {
    let mut input = ParserInput::new("");
    let mut parser = Parser::new(&mut input);
    let result = FontVariationSettings::from_css(&mut parser);
    assert!(result.is_err()); // Empty input should fail
  }
}
