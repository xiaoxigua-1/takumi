//! Length units and measurement types for the takumi styling system.
//!
//! This module provides various length units (px, em, rem, %, vh, vw) and
//! utility types for handling measurements and spacing in CSS-like layouts.

use serde::{Deserialize, Serialize};
use taffy::{CompactLength, Dimension, LengthPercentage, LengthPercentageAuto, Rect, Size};
use ts_rs::TS;

use crate::core::viewport::RenderContext;

/// Represents a value that can be a specific length, percentage, or automatic.
///
/// This corresponds to CSS values that can be specified as pixels, percentages,
/// or the 'auto' keyword for automatic sizing.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Copy, TS)]
#[serde(rename_all = "kebab-case")]
pub enum LengthUnit {
  /// Automatic sizing based on content
  Auto,
  /// Minimum content size
  MinContent,
  /// Maximum content size
  MaxContent,
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
}

impl From<f32> for LengthUnit {
  fn from(value: f32) -> Self {
    Self::Px(value)
  }
}

impl Default for LengthUnit {
  fn default() -> Self {
    Self::Auto
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
      LengthUnit::MinContent => CompactLength::min_content(),
      LengthUnit::MaxContent => CompactLength::max_content(),
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
      LengthUnit::Auto | LengthUnit::MinContent | LengthUnit::MaxContent => 0.0,
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
    // SAFETY: only length/percentage/auto are allowed
    unsafe { Dimension::from_raw(self.to_compact_length(context)) }
  }
}

/// Represents values that can be applied to all sides of an element.
///
/// This enum allows for flexible specification of values like padding, margin,
/// or border sizes using either a single value for all sides, separate values
/// for vertical/horizontal axes, or individual values for each side.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, TS, PartialEq)]
#[serde(untagged)]
pub enum SidesValue<T> {
  /// Same value for all four sides
  SingleValue(T),
  /// Separate values for vertical and horizontal sides (vertical, horizontal)
  AxisSidesArray(T, T),
  /// Individual values for each side (top, right, bottom, left)
  AllSides(T, T, T, T),
}

impl<T> From<T> for SidesValue<T> {
  fn from(value: T) -> Self {
    Self::SingleValue(value)
  }
}

impl<T: Default> Default for SidesValue<T> {
  fn default() -> Self {
    Self::SingleValue(T::default())
  }
}

impl<T: Copy, F: Copy + Default + Into<T>> From<SidesValue<F>> for Rect<T> {
  fn from(value: SidesValue<F>) -> Self {
    match value {
      SidesValue::AllSides(top, right, bottom, left) => Rect {
        left: left.into(),
        right: right.into(),
        top: top.into(),
        bottom: bottom.into(),
      },
      SidesValue::AxisSidesArray(vertical, horizontal) => Rect {
        left: horizontal.into(),
        right: horizontal.into(),
        top: vertical.into(),
        bottom: vertical.into(),
      },
      SidesValue::SingleValue(value) => Rect {
        left: value.into(),
        right: value.into(),
        top: value.into(),
        bottom: value.into(),
      },
    }
  }
}

/// Represents spacing between flex items.
///
/// Can be either a single value applied to both axes, or separate values
/// for horizontal and vertical spacing.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, TS, PartialEq)]
#[serde(untagged)]
pub enum Gap {
  /// Same gap value for both horizontal and vertical spacing
  SingleValue(LengthUnit),
  /// Separate values for horizontal and vertical spacing (horizontal, vertical)
  Array(LengthUnit, LengthUnit),
}

impl Default for Gap {
  fn default() -> Self {
    Self::SingleValue(LengthUnit::Px(0.0))
  }
}

impl Gap {
  /// Resolves the gap to a size in length percentages.
  ///
  /// This method converts the gap value to a size in length percentages,
  /// which can be used to set the size of flex items in a flex container.
  pub fn resolve_to_size(self, context: &RenderContext) -> Size<LengthPercentage> {
    match self {
      Gap::SingleValue(value) => Size {
        width: value.resolve_to_length_percentage(context),
        height: value.resolve_to_length_percentage(context),
      },
      Gap::Array(horizontal, vertical) => Size {
        width: horizontal.resolve_to_length_percentage(context),
        height: vertical.resolve_to_length_percentage(context),
      },
    }
  }
}

/// Represents values for horizontal and vertical axes.
///
/// Used for properties that can have different values for horizontal
/// and vertical directions, such as padding or margin.
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct AxisSides<T> {
  /// Horizontal axis value
  #[serde(default)]
  pub horizontal: T,
  /// Vertical axis value
  #[serde(default)]
  pub vertical: T,
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
