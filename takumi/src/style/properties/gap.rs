use cssparser::{Parser, ParserInput};
use serde::{Deserialize, Serialize};
use taffy::{LengthPercentage, Size};
use ts_rs::TS;

use crate::{FromCss, LengthUnit, RenderContext};

/// Represents spacing between flex items.
///
/// Can be either a single value applied to both axes, or separate values
/// for horizontal and vertical spacing.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, TS, PartialEq)]
#[serde(try_from = "GapValue")]
#[ts(as = "GapValue")]
pub struct Gap(pub LengthUnit, pub LengthUnit);

/// Represents a value for the gap property.
///
/// Can be either a single value applied to both axes, or separate values
/// for horizontal and vertical spacing.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(untagged)]
pub enum GapValue {
  /// Same gap value for both horizontal and vertical spacing
  SingleValue(LengthUnit),
  /// Separate values for horizontal and vertical spacing (horizontal, vertical)
  Array(LengthUnit, LengthUnit),
  /// CSS string representation
  Css(String),
}

impl Default for Gap {
  fn default() -> Self {
    Self(LengthUnit::Px(0.0), LengthUnit::Px(0.0))
  }
}

impl TryFrom<GapValue> for Gap {
  type Error = &'static str;

  fn try_from(value: GapValue) -> Result<Self, Self::Error> {
    match value {
      GapValue::SingleValue(value) => Ok(Self(value, value)),
      GapValue::Array(horizontal, vertical) => Ok(Self(horizontal, vertical)),
      GapValue::Css(value) => {
        let mut input = ParserInput::new(&value);
        let mut parser = Parser::new(&mut input);

        let first = LengthUnit::from_css(&mut parser).map_err(|_| "Failed to parse CSS gap")?;

        if let Ok(second) = LengthUnit::from_css(&mut parser) {
          Ok(Self(first, second))
        } else {
          Ok(Self(first, first))
        }
      }
    }
  }
}

impl Gap {
  /// Resolves the gap to a size in length percentages.
  ///
  /// This method converts the gap value to a size in length percentages,
  /// which can be used to set the size of flex items in a flex container.
  pub fn resolve_to_size(self, context: &RenderContext) -> Size<LengthPercentage> {
    Size {
      height: self.0.resolve_to_length_percentage(context),
      width: self.1.resolve_to_length_percentage(context),
    }
  }
}
