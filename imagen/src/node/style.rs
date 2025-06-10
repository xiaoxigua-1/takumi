use cosmic_text::{Align, Weight};
use merge::{Merge, option::overwrite_none};
use serde::{Deserialize, Serialize};
use taffy::{
  AlignItems, Dimension, Display, FlexDirection, JustifyContent, LengthPercentage,
  LengthPercentageAuto, Position, Rect, Size, style::Style as TaffyStyle,
};

use crate::color::Color;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct FontWeight(pub u16);

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

impl From<Color> for Background {
  fn from(color: Color) -> Self {
    Background::Color(color)
  }
}

impl Default for ObjectFit {
  fn default() -> Self {
    Self::Contain
  }
}

#[derive(Debug, Clone, Deserialize, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum TextAlign {
  Left,
  Right,
  Center,
  Justified,
  End,
}

impl From<TextAlign> for Align {
  fn from(value: TextAlign) -> Self {
    match value {
      TextAlign::Left => Align::Left,
      TextAlign::Right => Align::Right,
      TextAlign::Center => Align::Center,
      TextAlign::Justified => Align::Justified,
      TextAlign::End => Align::End,
    }
  }
}

impl Default for TextAlign {
  fn default() -> Self {
    Self::Left
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Style {
  pub width: ValuePercentageAuto,
  pub height: ValuePercentageAuto,
  pub padding: SidesValue<ValuePercentageAuto>,
  pub margin: SidesValue<ValuePercentageAuto>,
  pub inset: SidesValue<ValuePercentageAuto>,
  pub flex_direction: FlexDirection,
  pub justify_content: Option<JustifyContent>,
  pub align_items: Option<AlignItems>,
  pub position: Position,
  pub gap: Gap,
  pub flex_grow: f32,
  pub border_size: SidesValue<ValuePercentageAuto>,
  pub object_fit: Option<ObjectFit>,
  pub background: Option<Background>,

  #[serde(flatten)]
  pub inheritable_style: InheritableStyle,
}

impl Default for Style {
  fn default() -> Self {
    Self {
      margin: SidesValue::SingleValue(ValuePercentageAuto::SpecificValue(0.0)),
      width: Default::default(),
      height: Default::default(),
      padding: Default::default(),
      inset: Default::default(),
      flex_direction: Default::default(),
      justify_content: Default::default(),
      align_items: Default::default(),
      position: Default::default(),
      gap: Default::default(),
      flex_grow: Default::default(),
      border_size: Default::default(),
      object_fit: Default::default(),
      background: Default::default(),
      inheritable_style: Default::default(),
    }
  }
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
  pub text_align: Option<TextAlign>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FontStyle {
  pub font_size: f32,
  pub font_family: Option<String>,
  pub text_align: TextAlign,
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
      text_align: TextAlign::default(),
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
      text_align: style.inheritable_style.text_align.unwrap_or_default(),
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Gap {
  SingleValue(f32),
  Array(f32, f32),
}

impl Default for Gap {
  fn default() -> Self {
    Self::SingleValue(0.0)
  }
}

impl From<Gap> for Size<LengthPercentage> {
  fn from(gap: Gap) -> Self {
    match gap {
      Gap::SingleValue(value) => Size {
        width: LengthPercentage::length(value),
        height: LengthPercentage::length(value),
      },
      Gap::Array(horizontal, vertical) => Size {
        width: LengthPercentage::length(horizontal),
        height: LengthPercentage::length(vertical),
      },
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Border {
  pub color: Option<Color>,
  pub size: SidesValue<f32>,
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

#[derive(Debug, Clone, Deserialize, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ValuePercentageAuto {
  Auto,
  Percentage(f32),
  #[serde(untagged)]
  SpecificValue(f32),
}

impl Default for ValuePercentageAuto {
  fn default() -> Self {
    Self::Auto
  }
}

impl From<ValuePercentageAuto> for Dimension {
  fn from(value: ValuePercentageAuto) -> Self {
    match value {
      ValuePercentageAuto::Auto => Dimension::auto(),
      ValuePercentageAuto::SpecificValue(value) => Dimension::length(value),
      ValuePercentageAuto::Percentage(value) => Dimension::percent(value),
    }
  }
}

impl From<ValuePercentageAuto> for LengthPercentageAuto {
  fn from(value: ValuePercentageAuto) -> Self {
    match value {
      ValuePercentageAuto::Auto => LengthPercentageAuto::auto(),
      ValuePercentageAuto::SpecificValue(value) => LengthPercentageAuto::length(value),
      ValuePercentageAuto::Percentage(value) => LengthPercentageAuto::percent(value),
    }
  }
}

impl From<ValuePercentageAuto> for LengthPercentage {
  fn from(value: ValuePercentageAuto) -> Self {
    match value {
      ValuePercentageAuto::Auto => LengthPercentage::length(0.0),
      ValuePercentageAuto::SpecificValue(value) => LengthPercentage::length(value),
      ValuePercentageAuto::Percentage(value) => LengthPercentage::percent(value),
    }
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
      margin: style.margin.into(),
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
