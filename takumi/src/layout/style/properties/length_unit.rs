//! Length units and measurement types for the takumi styling system.
//!
//! This module provides various length units (px, em, rem, %, vh, vw) and
//! utility types for handling measurements and spacing in CSS-like layouts.

use cssparser::{Parser, ParserInput, Token, match_ignore_ascii_case};
use serde::{Deserialize, Serialize};
use taffy::{CompactLength, Dimension, LengthPercentage, LengthPercentageAuto, Rect};
use ts_rs::TS;

use crate::{
  layout::style::{FromCss, ParseResult},
  rendering::RenderContext,
};

/// Represents a value that can be a specific length, percentage, or automatic.
///
/// This corresponds to CSS values that can be specified as pixels, percentages,
/// or the 'auto' keyword for automatic sizing.
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, Copy, TS)]
#[serde(try_from = "LengthUnitValue", into = "LengthUnitValue")]
#[ts(as = "LengthUnitValue")]
pub enum LengthUnit {
  /// Automatic sizing based on content
  #[default]
  Auto,
  /// Percentage value relative to parent container (0-100)
  Percentage(f32),
  /// Rem value relative to the root font size
  Rem(f32),
  /// Em value relative to the font size
  Em(f32),
  /// Vh value relative to the viewport height (0-100)
  Vh(f32),
  /// Vw value relative to the viewport width (0-100)
  Vw(f32),
  /// Specific pixel value
  Px(f32),
}

/// Proxy type for CSS `LengthUnit` serialization/deserialization.
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum LengthUnitValue {
  /// Automatic sizing based on content
  Auto,
  /// Percentage value relative to parent container (0-100)
  Percentage(f32),
  /// Rem value relative to the root font size
  Rem(f32),
  /// Em value relative to the font size
  Em(f32),
  /// Vh value relative to the viewport height (0-100)
  Vh(f32),
  /// Vw value relative to the viewport width (0-100)
  Vw(f32),
  /// Specific pixel value
  #[serde(untagged)]
  Px(f32),
  /// CSS string representation
  #[serde(untagged)]
  Css(String),
}

impl TryFrom<LengthUnitValue> for LengthUnit {
  type Error = &'static str;

  fn try_from(value: LengthUnitValue) -> Result<Self, Self::Error> {
    match value {
      LengthUnitValue::Auto => Ok(Self::Auto),
      LengthUnitValue::Percentage(v) => Ok(Self::Percentage(v)),
      LengthUnitValue::Rem(v) => Ok(Self::Rem(v)),
      LengthUnitValue::Em(v) => Ok(Self::Em(v)),
      LengthUnitValue::Vh(v) => Ok(Self::Vh(v)),
      LengthUnitValue::Vw(v) => Ok(Self::Vw(v)),
      LengthUnitValue::Px(v) => Ok(Self::Px(v)),
      LengthUnitValue::Css(s) => {
        let mut input = ParserInput::new(&s);
        let mut parser = Parser::new(&mut input);

        let unit =
          LengthUnit::from_css(&mut parser).map_err(|_| "Failed to parse CSS length unit")?;

        // Ensure no trailing tokens remain so that multi-value CSS like
        // "1px 2px" does not get parsed as a single LengthUnit.
        parser
          .expect_exhausted()
          .map_err(|_| "Failed to parse CSS length unit: trailing tokens found")?;

        Ok(unit)
      }
    }
  }
}

impl From<LengthUnit> for LengthUnitValue {
  fn from(value: LengthUnit) -> Self {
    match value {
      LengthUnit::Auto => LengthUnitValue::Auto,
      LengthUnit::Percentage(v) => LengthUnitValue::Percentage(v),
      LengthUnit::Rem(v) => LengthUnitValue::Rem(v),
      LengthUnit::Em(v) => LengthUnitValue::Em(v),
      LengthUnit::Vh(v) => LengthUnitValue::Vh(v),
      LengthUnit::Vw(v) => LengthUnitValue::Vw(v),
      LengthUnit::Px(v) => LengthUnitValue::Px(v),
    }
  }
}

impl LengthUnit {
  /// Returns a zero pixel length unit.
  pub const fn zero() -> Self {
    Self::Px(0.0)
  }
}

impl From<f32> for LengthUnit {
  fn from(value: f32) -> Self {
    Self::Px(value)
  }
}

impl<'i> FromCss<'i> for LengthUnit {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    let location = input.current_source_location();
    let token = input.next()?;

    match *token {
      Token::Ident(ref unit) => match_ignore_ascii_case! {&unit,
        "auto" => Ok(Self::Auto),
        _ => Err(location.new_basic_unexpected_token_error(token.clone()).into()),
      },
      Token::Dimension {
        value, ref unit, ..
      } => {
        match_ignore_ascii_case! {&unit,
          "px" => Ok(Self::Px(value)),
          "em" => Ok(Self::Em(value)),
          "rem" => Ok(Self::Rem(value)),
          "vw" => Ok(Self::Vw(value)),
          "vh" => Ok(Self::Vh(value)),
          _ => Err(location.new_basic_unexpected_token_error(token.clone()).into()),
        }
      }
      Token::Percentage { unit_value, .. } => Ok(Self::Percentage(unit_value * 100.0)),
      Token::Number { value, .. } => Ok(Self::Px(value)),
      _ => Err(
        location
          .new_basic_unexpected_token_error(token.clone())
          .into(),
      ),
    }
  }
}

impl LengthUnit {
  /// Converts the length unit to a compact length representation.
  ///
  /// This method converts the length unit (either a percentage, pixel, rem, em, vh, vw, or auto)
  /// into a compact length format that can be used by the layout engine.
  pub fn to_compact_length(self, context: &RenderContext) -> CompactLength {
    match self {
      LengthUnit::Auto => CompactLength::auto(),
      LengthUnit::Px(value) => CompactLength::length(value),
      LengthUnit::Percentage(value) => CompactLength::percent(value / 100.0),
      LengthUnit::Rem(value) => CompactLength::length(value * context.viewport.font_size),
      LengthUnit::Em(value) => CompactLength::length(value * context.parent_font_size),
      LengthUnit::Vh(value) => {
        CompactLength::length(context.viewport.height as f32 * value / 100.0)
      }
      LengthUnit::Vw(value) => CompactLength::length(context.viewport.width as f32 * value / 100.0),
    }
  }

  /// Resolves the length unit to a `LengthPercentage`.
  pub fn resolve_to_length_percentage(self, context: &RenderContext) -> LengthPercentage {
    let compact_length = self.to_compact_length(context);

    if compact_length.is_auto() {
      return LengthPercentage::length(0.0);
    }

    // SAFETY: only length/percentage are allowed
    unsafe { LengthPercentage::from_raw(compact_length) }
  }

  /// Resolves the length unit to a pixel value.
  pub fn resolve_to_px(self, context: &RenderContext) -> f32 {
    match self {
      LengthUnit::Auto => 0.0,
      LengthUnit::Px(value) => value,
      LengthUnit::Percentage(value) => value * context.parent_font_size / 100.0,
      LengthUnit::Rem(value) => value * context.viewport.font_size,
      LengthUnit::Em(value) => value * context.parent_font_size,
      LengthUnit::Vh(value) => value * context.viewport.height as f32 / 100.0,
      LengthUnit::Vw(value) => value * context.viewport.width as f32 / 100.0,
    }
  }

  /// Resolves the length unit to a `LengthPercentageAuto`.
  pub fn resolve_to_length_percentage_auto(self, context: &RenderContext) -> LengthPercentageAuto {
    // SAFETY: only length/percentage/auto are allowed
    unsafe { LengthPercentageAuto::from_raw(self.to_compact_length(context)) }
  }

  /// Resolves the length unit to a `Dimension`.
  pub fn resolve_to_dimension(self, context: &RenderContext) -> Dimension {
    self.resolve_to_length_percentage_auto(context).into()
  }
}
/// Utility function to resolve a rect of length units to length percentages.
pub fn resolve_length_unit_rect_to_length_percentage(
  context: &RenderContext,
  value: Rect<LengthUnit>,
) -> Rect<LengthPercentage> {
  Rect {
    left: value.left.resolve_to_length_percentage(context),
    right: value.right.resolve_to_length_percentage(context),
    top: value.top.resolve_to_length_percentage(context),
    bottom: value.bottom.resolve_to_length_percentage(context),
  }
}

/// Utility function to resolve a rect of length units to length percentage auto.
pub fn resolve_length_unit_rect_to_length_percentage_auto(
  context: &RenderContext,
  value: Rect<LengthUnit>,
) -> Rect<LengthPercentageAuto> {
  Rect {
    left: value.left.resolve_to_length_percentage_auto(context),
    right: value.right.resolve_to_length_percentage_auto(context),
    top: value.top.resolve_to_length_percentage_auto(context),
    bottom: value.bottom.resolve_to_length_percentage_auto(context),
  }
}
