use cosmic_text::Weight;
use merge::{option::overwrite_none, Merge};
use serde::{Deserialize, Serialize};
use taffy::{
  AlignItems, Dimension, Display, FlexDirection, JustifyContent, LengthPercentage,
  LengthPercentageAuto, Position, Rect, Size, prelude::FromLength, style::Style as TaffyStyle,
  style_helpers::TaffyZero,
};

use crate::color::Color;

#[derive(Debug, Clone, Deserialize, Default, Copy, Serialize)]
#[serde(transparent)]
pub struct Length(pub f32);

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct FontWeight(u16);

impl Default for FontWeight {
  fn default() -> Self {
    FontWeight(Weight::NORMAL.0)
  }
}

impl From<FontWeight> for Weight {
  fn from(weight: FontWeight) -> Self {
    Weight(weight.0)
  }
}

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
    LengthPercentage::length(value.0)
  }
}

impl From<Length> for LengthPercentageAuto {
  fn from(value: Length) -> Self {
    LengthPercentageAuto::from_length(value.0)
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ObjectFit {
  Contain,
  Cover,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Background {
  Image(String),
  Color(Color),
}

impl Default for ObjectFit {
  fn default() -> Self {
    Self::Contain
  }
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(default)]
pub struct Style {
  pub width: ValueOrAutoFull<Length>,
  pub height: ValueOrAutoFull<Length>,
  pub padding: SidesValue<Length>,
  pub margin: SidesValue<Length>,
  pub inset: SidesValue<Length>,
  pub flex_direction: FlexDirection,
  pub justify_content: Option<JustifyContent>,
  pub align_items: Option<AlignItems>,
  pub position: Position,
  pub gap: Gap,
  pub flex_grow: f32,
  pub border_size: SidesValue<Length>,
  pub object_fit: Option<ObjectFit>,
  pub background: Option<Background>,

  #[serde(flatten)]
  pub inheritable_style: InheritableStyle,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize, Merge)]
#[merge(strategy = overwrite_none)]
pub struct InheritableStyle {
  pub border_color: Option<Color>,
  pub color: Option<Color>,
  pub font_size: Option<f32>,
  pub font_family: Option<String>,
  pub line_height: Option<f32>,
  pub font_weight: Option<FontWeight>,
  pub max_lines: Option<u32>,
  pub border_radius: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FontStyle {
  pub font_size: f32,
  pub font_family: Option<String>,
  pub line_height: f32,
  pub font_weight: FontWeight,
  pub max_lines: Option<u32>,
  pub color: Color,
}

impl Default for FontStyle {
  fn default() -> Self {
    Self {
      font_size: 16.0,
      font_family: None,
      line_height: 1.0,
      font_weight: FontWeight::default(),
      max_lines: None,
      color: Color::default(),
    }
  }
}

impl From<&Style> for FontStyle {
  fn from(style: &Style) -> Self {
    Self {
      font_size: style.inheritable_style.font_size.unwrap_or(16.0),
      font_family: style.inheritable_style.font_family.clone(),
      line_height: style.inheritable_style.line_height.unwrap_or(1.0),
      font_weight: style.inheritable_style.font_weight.unwrap_or_default(),
      max_lines: style.inheritable_style.max_lines,
      color: style.inheritable_style.color.unwrap_or_default(),
    }
  }
}

impl InheritableStyle {
  pub fn inherit_from(&self, parent: &InheritableStyle) -> Self {
    Self {
      border_color: self.border_color.or(parent.border_color),
      color: self.color.or(parent.color),
      font_size: self.font_size.or(parent.font_size),
      font_family: self.font_family.clone().or_else(|| parent.font_family.clone()),
      line_height: self.line_height.or(parent.line_height),
      font_weight: self.font_weight.or(parent.font_weight),
      max_lines: self.max_lines.or(parent.max_lines),
      border_radius: self.border_radius.or(parent.border_radius),
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
        width: LengthPercentage::length(value.into()),
        height: LengthPercentage::length(value.into()),
      },
      Gap::Array(horizontal, vertical) => Size {
        width: LengthPercentage::length(horizontal.into()),
        height: LengthPercentage::length(vertical.into()),
      },
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Border {
  pub color: Option<Color>,
  pub size: SidesValue<Length>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SidesValue<T: Default> {
  SingleValue(T),
  AxisSidesArray(T, T),
  AllSides(T, T, T, T),
}

impl<T: Default> Default for SidesValue<T> {
  fn default() -> Self {
    Self::SingleValue(T::default())
  }
}

impl<T: FromLength + Copy + TaffyZero> From<SidesValue<Length>> for Rect<T> {
  fn from(value: SidesValue<Length>) -> Self {
    match value {
      SidesValue::AllSides(top, right, bottom, left) => Rect {
        left: T::from_length(left),
        right: T::from_length(right),
        top: T::from_length(top),
        bottom: T::from_length(bottom),
      },
      SidesValue::AxisSidesArray(vertical, horizontal) => Rect {
        left: T::from_length(horizontal),
        right: T::from_length(horizontal),
        top: T::from_length(vertical),
        bottom: T::from_length(vertical),
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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
      ValueOrAutoFull::SpecificValue(value) => Dimension::length(value.into()),
      ValueOrAutoFull::Auto => Dimension::auto(),
      ValueOrAutoFull::Full => Dimension::percent(1.0),
    }
  }
}

impl From<ValueOrAutoFull<Length>> for LengthPercentageAuto {
  fn from(value: ValueOrAutoFull<Length>) -> Self {
    match value {
      ValueOrAutoFull::SpecificValue(value) => LengthPercentageAuto::length(value.into()),
      ValueOrAutoFull::Auto => LengthPercentageAuto::auto(),
      ValueOrAutoFull::Full => LengthPercentageAuto::percent(1.0),
    }
  }
}

impl<T> Default for ValueOrAutoFull<T> {
  fn default() -> Self {
    Self::Auto
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AxisSides<T> {
  #[serde(default)]
  pub horizontal: T,
  #[serde(default)]
  pub vertical: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
        width: style.width.into(),
        height: style.height.into(),
      },
      border: style.border_size.into(),
      padding: style.padding.into(),
      inset: style.inset.into(),
      display: Display::Flex,
      flex_direction: style.flex_direction,
      position: style.position,
      justify_content: style.justify_content,
      flex_grow: style.flex_grow,
      align_items: style.align_items,
      gap: style.gap.into(),
      ..Default::default()
    }
  }
}
