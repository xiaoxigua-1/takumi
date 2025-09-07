use cssparser::{Parser, Token};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::layout::style::properties::{FromCss, ParseResult};

/// Represents a single font feature setting with a feature tag and value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct FontFeatureSetting {
  /// The four-character OpenType feature tag (e.g., "liga", "kern", "smcp")
  pub tag: String,
  /// The numeric value for this feature (usually 0 or 1, but can be other values)
  pub value: u16,
}

/// Controls OpenType font features via CSS font-feature-settings property.
///
/// This allows enabling/disabling specific typographic features in OpenType fonts
/// such as ligatures, kerning, small caps, and other advanced typography features.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
pub struct FontFeatureSettings(pub Vec<FontFeatureSetting>);

impl<'i> FromCss<'i> for FontFeatureSettings {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    let mut settings = Vec::new();

    // Handle "normal" value
    if input
      .try_parse(|i| i.expect_ident_matching("normal"))
      .is_ok()
    {
      return Ok(FontFeatureSettings::default());
    }

    loop {
      // Parse feature tag (string or identifier)
      let location = input.current_source_location();
      let tag = match input.next()? {
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

      // Validate tag (should be 4 characters for standard OpenType features, but can be other lengths)
      if tag.is_empty() {
        let location = input.current_source_location();
        return Err(
          location
            .new_basic_unexpected_token_error(Token::Ident("".into()))
            .into(),
        );
      }

      // Parse optional value (defaults to 1 if not specified)
      let value = input
        .try_parse(Parser::expect_integer)
        .map(|value| value as u16)
        .unwrap_or(1);

      settings.push(FontFeatureSetting {
        tag: tag.to_string(),
        value,
      });

      // Check for comma (more settings) or end
      if input.try_parse(Parser::expect_comma).is_err() {
        break;
      }
    }

    Ok(FontFeatureSettings(settings))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cssparser::ParserInput;

  #[test]
  fn test_font_feature_settings_parsing() {
    let test_cases = vec![
      ("\"liga\" 1", vec![("liga", 1)]),
      ("\"kern\" 0, \"liga\" 1", vec![("kern", 0), ("liga", 1)]),
      ("liga, kern 0", vec![("liga", 1), ("kern", 0)]), // default value of 1 for liga
      ("\"smcp\" 1, \"onum\" 1", vec![("smcp", 1), ("onum", 1)]),
      ("normal", vec![]), // normal should result in empty settings
    ];

    for (input, expected) in test_cases {
      let mut input = ParserInput::new(input);
      let mut parser = Parser::new(&mut input);
      let result = FontFeatureSettings::from_css(&mut parser).unwrap();

      assert_eq!(result.0.len(), expected.len());
      for (i, (tag, value)) in expected.iter().enumerate() {
        assert_eq!(result.0[i].tag, *tag);
        assert_eq!(result.0[i].value, *value);
      }
    }
  }

  #[test]
  fn test_font_feature_settings_empty() {
    let mut input = ParserInput::new("");
    let mut parser = Parser::new(&mut input);
    let result = FontFeatureSettings::from_css(&mut parser);
    assert!(result.is_err()); // Empty input should fail
  }
}
