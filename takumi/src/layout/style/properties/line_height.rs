use cssparser::{Parser, ParserInput};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
  layout::style::{FromCss, LengthUnit, ParseResult},
  rendering::RenderContext,
};

/// Represents a line height value, number value is parsed as em.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TS, Copy)]
#[serde(try_from = "LineHeightValue")]
#[ts(as = "LineHeightValue")]
pub struct LineHeight(pub LengthUnit);

/// Proxy type for `LineHeight` Css deserialization.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TS)]
#[serde(untagged)]
pub enum LineHeightValue {
  /// A number value.
  Number(f32),
  /// A CSS string value.
  Css(String),
  /// A length value.
  Length(LengthUnit),
}

impl TryFrom<LineHeightValue> for LineHeight {
  type Error = String;

  fn try_from(value: LineHeightValue) -> Result<Self, Self::Error> {
    match value {
      LineHeightValue::Number(number) => Ok(LineHeight(LengthUnit::Em(number))),
      LineHeightValue::Css(css) => {
        let mut input = ParserInput::new(&css);
        let mut parser = Parser::new(&mut input);

        LineHeight::from_css(&mut parser).map_err(|e| e.to_string())
      }
      LineHeightValue::Length(length) => Ok(LineHeight(length)),
    }
  }
}

impl<'i> FromCss<'i> for LineHeight {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    let Ok(number) = input.try_parse(Parser::expect_number) else {
      return LengthUnit::from_css(input).map(LineHeight);
    };

    Ok(LineHeight(LengthUnit::Em(number)))
  }
}

impl LineHeight {
  /// Converts the line height to a parley line height.
  pub fn into_parley(self, context: &RenderContext) -> parley::LineHeight {
    match self.0 {
      LengthUnit::Px(value) => parley::LineHeight::Absolute(value),
      LengthUnit::Em(value) => parley::LineHeight::FontSizeRelative(value),
      LengthUnit::Percentage(value) => parley::LineHeight::FontSizeRelative(value / 100.0),
      unit => parley::LineHeight::Absolute(unit.resolve_to_px(context, context.parent_font_size)),
    }
  }
}
