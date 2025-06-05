use serde::Deserialize;
use taffy::{
  AlignItems, Dimension, Display, FlexDirection, JustifyContent, LengthPercentage,
  LengthPercentageAuto, Position, Rect, Size, style::Style as TaffyStyle, style_helpers::TaffyZero,
};

use crate::color::Color;

#[derive(Debug, Clone, Deserialize, Default, Copy)]
#[serde(transparent)]
pub struct Length(f32);

impl TaffyZero for Length {
  const ZERO: Self = Length(0.0);
}

impl From<Length> for f32 {
  fn from(value: Length) -> Self {
    value.0
  }
}

impl From<f32> for Length {
  fn from(value: f32) -> Self {
    Length(value)
  }
}

impl From<Length> for LengthPercentage {
  fn from(value: Length) -> Self {
    LengthPercentage::Length(value.0)
  }
}

impl From<Length> for LengthPercentageAuto {
  fn from(value: Length) -> Self {
    LengthPercentageAuto::Length(value.0)
  }
}

// Helper trait to mark types that can be converted from Length
pub trait FromLength {
  fn from_length(value: Length) -> Self;
}

impl FromLength for LengthPercentage {
  fn from_length(value: Length) -> Self {
    value.into()
  }
}

impl FromLength for LengthPercentageAuto {
  fn from_length(value: Length) -> Self {
    value.into()
  }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct Style {
  pub width: Option<ValueOrAutoFull<Length>>,
  pub height: Option<ValueOrAutoFull<Length>>,
  pub background_color: Option<Color>,
  pub border: Option<Border>,
  pub padding: Option<SidesValue<Length>>,
  pub margin: Option<SidesValue<Length>>,
  pub top: ValueOrAutoFull<Length>,
  pub left: ValueOrAutoFull<Length>,
  pub right: ValueOrAutoFull<Length>,
  pub bottom: ValueOrAutoFull<Length>,
  pub flex_direction: FlexDirection,
  pub justify_content: Option<JustifyContent>,
  pub align_items: Option<AlignItems>,
  pub position: Position,
  pub gap: Gap,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Gap {
  SingleValue(Length),
  Array(Length, Length),
}

impl Default for Gap {
  fn default() -> Self {
    Self::SingleValue(Length(0.0))
  }
}

impl From<Gap> for Size<LengthPercentage> {
  fn from(gap: Gap) -> Self {
    match gap {
      Gap::SingleValue(value) => Size {
        width: LengthPercentage::Length(value.into()),
        height: LengthPercentage::Length(value.into()),
      },
      Gap::Array(horizontal, vertical) => Size {
        width: LengthPercentage::Length(horizontal.into()),
        height: LengthPercentage::Length(vertical.into()),
      },
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Border {
  pub color: Option<Color>,
  pub size: SidesValue<Length>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum SidesValue<T: Default> {
  AllSides(IndividualSides<T>),
  AxisSides(AxisSides<T>),
  SingleValue(T),
}

impl<T: Default> Default for SidesValue<T> {
  fn default() -> Self {
    Self::SingleValue(T::default())
  }
}

impl<T: FromLength + Copy + TaffyZero> From<SidesValue<Length>> for Rect<T> {
  fn from(value: SidesValue<Length>) -> Self {
    match value {
      SidesValue::AllSides(sides) => Rect {
        left: T::from_length(sides.left),
        right: T::from_length(sides.right),
        top: T::from_length(sides.top),
        bottom: T::from_length(sides.bottom),
      },
      SidesValue::AxisSides(sides) => Rect {
        left: T::from_length(sides.horizontal),
        right: T::from_length(sides.horizontal),
        top: T::from_length(sides.vertical),
        bottom: T::from_length(sides.vertical),
      },
      SidesValue::SingleValue(value) => Rect {
        left: T::from_length(value),
        right: T::from_length(value),
        top: T::from_length(value),
        bottom: T::from_length(value),
      },
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValueOrAutoFull<T> {
  Full,
  Auto,
  #[serde(untagged)]
  SpecificValue(T),
}

impl From<ValueOrAutoFull<Length>> for Dimension {
  fn from(value: ValueOrAutoFull<Length>) -> Self {
    match value {
      ValueOrAutoFull::SpecificValue(value) => Dimension::Length(value.into()),
      ValueOrAutoFull::Auto => Dimension::Auto,
      ValueOrAutoFull::Full => Dimension::Percent(1.0),
    }
  }
}

impl From<ValueOrAutoFull<Length>> for LengthPercentageAuto {
  fn from(value: ValueOrAutoFull<Length>) -> Self {
    match value {
      ValueOrAutoFull::SpecificValue(value) => LengthPercentageAuto::Length(value.into()),
      ValueOrAutoFull::Auto => LengthPercentageAuto::Auto,
      ValueOrAutoFull::Full => LengthPercentageAuto::Percent(1.0),
    }
  }
}

impl<T> Default for ValueOrAutoFull<T> {
  fn default() -> Self {
    Self::Auto
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AxisSides<T> {
  #[serde(default)]
  pub horizontal: T,
  #[serde(default)]
  pub vertical: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IndividualSides<T> {
  #[serde(default)]
  pub top: T,
  #[serde(default)]
  pub right: T,
  #[serde(default)]
  pub bottom: T,
  #[serde(default)]
  pub left: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Values<T> {
  #[serde(default)]
  pub top: T,
  #[serde(default)]
  pub right: T,
  #[serde(default)]
  pub bottom: T,
  #[serde(default)]
  pub left: T,
}

impl From<Style> for TaffyStyle {
  fn from(style: Style) -> Self {
    Self {
      size: Size {
        width: style.width.unwrap_or_default().into(),
        height: style.height.unwrap_or_default().into(),
      },
      border: style
        .border
        .map(|border| border.size)
        .unwrap_or_default()
        .into(),
      padding: style.padding.unwrap_or_default().into(),
      inset: Rect {
        top: style.top.into(),
        right: style.right.into(),
        bottom: style.bottom.into(),
        left: style.left.into(),
      },
      display: Display::Flex,
      flex_direction: style.flex_direction,
      position: style.position,
      justify_content: style.justify_content,
      align_items: style.align_items,
      gap: style.gap.into(),
      ..Default::default()
    }
  }
}
