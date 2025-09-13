use cssparser::{Parser, ParserInput};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::layout::style::{
  FromCss, ParseResult,
  properties::{Color, LengthUnit},
};

/// Represents the `text-stroke` shorthand which accepts a width and an optional color.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(untagged)]
pub(crate) enum TextStrokeValue {
  /// Structured representation when provided as JSON.
  #[serde(rename_all = "camelCase")]
  Structured { width: LengthUnit, color: Color },
  /// Raw CSS string representation.
  Css(String),
}

/// Parsed `text-stroke` value.
///
/// `color` is optional; when absent the element's `color` property should be used.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, TS)]
#[serde(try_from = "TextStrokeValue")]
pub struct TextStroke {
  /// Stroke width as a `LengthUnit`.
  pub width: LengthUnit,
  /// Optional stroke color.
  pub color: Option<Color>,
}

impl TryFrom<TextStrokeValue> for TextStroke {
  type Error = String;

  fn try_from(value: TextStrokeValue) -> Result<Self, Self::Error> {
    match value {
      TextStrokeValue::Structured { width, color } => Ok(TextStroke {
        width,
        color: Some(color),
      }),
      TextStrokeValue::Css(s) => {
        let mut input = ParserInput::new(&s);
        let mut parser = Parser::new(&mut input);

        // Parse width first
        let width = LengthUnit::from_css(&mut parser).map_err(|e| e.to_string())?;

        // Try parse optional color
        let color = parser.try_parse(Color::from_css).ok();

        Ok(TextStroke { width, color })
      }
    }
  }
}

impl<'i> FromCss<'i> for TextStroke {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    // Parse width first
    let width = LengthUnit::from_css(input)?;
    // Try optional color
    let color = input.try_parse(Color::from_css).ok();

    Ok(TextStroke { width, color })
  }
}
