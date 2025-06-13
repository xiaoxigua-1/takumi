use cosmic_text::{Align, Weight};
use merge::{Merge, option::overwrite_none};
use serde::{Deserialize, Serialize};
use taffy::{
  Dimension, Display, LengthPercentage, LengthPercentageAuto, Rect, Size,
  style::Style as TaffyStyle,
};
use ts_rs::TS;

use crate::color::ColorInput;

/// Represents font weight as a numeric value.
///
/// This wraps a u16 value that corresponds to CSS font-weight values
/// (e.g., 100 for thin, 400 for normal, 700 for bold, 900 for black).
#[derive(Debug, Copy, Clone, Deserialize, Serialize, TS)]
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

/// Defines how an image should be resized to fit its container.
///
/// Similar to CSS object-fit property.
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS)]
#[serde(rename_all = "snake_case")]
pub enum ObjectFit {
  /// Scale the image to fit within the container while preserving aspect ratio
  Contain,
  /// Scale the image to cover the entire container while preserving aspect ratio
  Cover,
  /// Fill the container with the image, potentially distorting it
  Fill,
  /// Scale the image down to fit within the container while preserving aspect ratio, but never scale up
  ScaleDown,
  /// Don't resize the image, display it at its natural size
  None,
}

impl Default for ObjectFit {
  fn default() -> Self {
    Self::Fill
  }
}

/// Text alignment options for text rendering.
///
/// Corresponds to CSS text-align property values.
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
  /// Align text to the left edge
  Left,
  /// Align text to the right edge
  Right,
  /// Center align text
  Center,
  /// Justify text to both edges
  Justify,
  /// Align text to the end (language-dependent)
  End,
}

impl From<TextAlign> for Align {
  fn from(value: TextAlign) -> Self {
    match value {
      TextAlign::Left => Align::Left,
      TextAlign::Right => Align::Right,
      TextAlign::Center => Align::Center,
      TextAlign::Justify => Align::Justified,
      TextAlign::End => Align::End,
    }
  }
}

impl Default for TextAlign {
  fn default() -> Self {
    Self::Left
  }
}

/// Defines the positioning method for an element.
///
/// This enum determines how an element is positioned within its containing element.
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS)]
#[serde(rename_all = "snake_case")]
pub enum Position {
  /// Element is positioned according to the normal flow of the document
  Relative,
  /// Element is positioned relative to its nearest positioned ancestor
  Absolute,
}

impl Default for Position {
  fn default() -> Self {
    Self::Relative
  }
}

impl From<Position> for taffy::style::Position {
  fn from(value: Position) -> Self {
    match value {
      Position::Relative => taffy::style::Position::Relative,
      Position::Absolute => taffy::style::Position::Absolute,
    }
  }
}

/// Defines the direction of flex items within a flex container.
///
/// This enum determines how flex items are laid out along the main axis.
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS)]
#[serde(rename_all = "snake_case")]
pub enum FlexDirection {
  /// Items are laid out horizontally from left to right
  Row,
  /// Items are laid out vertically from top to bottom
  Column,
  /// Items are laid out horizontally from right to left
  RowReverse,
  /// Items are laid out vertically from bottom to top
  ColumnReverse,
}

impl Default for FlexDirection {
  fn default() -> Self {
    Self::Row
  }
}

impl From<FlexDirection> for taffy::style::FlexDirection {
  fn from(value: FlexDirection) -> Self {
    match value {
      FlexDirection::Row => taffy::style::FlexDirection::Row,
      FlexDirection::Column => taffy::style::FlexDirection::Column,
      FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
      FlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
    }
  }
}

/// Defines how flex items are aligned along the main axis.
///
/// This enum determines how space is distributed between and around flex items
/// along the main axis of the flex container.
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS)]
#[serde(rename_all = "snake_case")]
pub enum JustifyContent {
  /// Items are packed toward the start of the flex-direction
  Start,
  /// Items are packed toward the end of the flex-direction
  End,
  /// Items are packed toward the start of the flex-direction
  FlexStart,
  /// Items are packed toward the end of the flex-direction
  FlexEnd,
  /// Items are centered along the main axis
  Center,
  /// Items are evenly distributed with the first item at the start and last at the end
  SpaceBetween,
  /// Items are evenly distributed with equal space around them
  SpaceAround,
}

impl From<JustifyContent> for taffy::style::JustifyContent {
  fn from(value: JustifyContent) -> Self {
    match value {
      JustifyContent::Start => taffy::style::JustifyContent::Start,
      JustifyContent::End => taffy::style::JustifyContent::End,
      JustifyContent::FlexStart => taffy::style::JustifyContent::FlexStart,
      JustifyContent::FlexEnd => taffy::style::JustifyContent::FlexEnd,
      JustifyContent::Center => taffy::style::JustifyContent::Center,
      JustifyContent::SpaceBetween => taffy::style::JustifyContent::SpaceBetween,
      JustifyContent::SpaceAround => taffy::style::JustifyContent::SpaceAround,
    }
  }
}

/// Defines how flex items are aligned along the cross axis.
///
/// This enum determines how flex items are aligned within the flex container
/// along the cross axis (perpendicular to the main axis).
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS)]
#[serde(rename_all = "snake_case")]
pub enum AlignItems {
  /// Items are aligned to the start of the cross axis
  Start,
  /// Items are aligned to the end of the cross axis
  End,
  /// Items are centered along the cross axis
  Center,
  /// Items are aligned such that their baselines align
  Baseline,
  /// Items are stretched to fill the container along the cross axis
  Stretch,
}

impl From<AlignItems> for taffy::style::AlignItems {
  fn from(value: AlignItems) -> Self {
    match value {
      AlignItems::Start => taffy::style::AlignItems::Start,
      AlignItems::End => taffy::style::AlignItems::End,
      AlignItems::Center => taffy::style::AlignItems::Center,
      AlignItems::Baseline => taffy::style::AlignItems::Baseline,
      AlignItems::Stretch => taffy::style::AlignItems::Stretch,
    }
  }
}

/// Main styling structure that contains all layout and visual properties.
///
/// This structure combines both layout properties (like width, height, padding)
/// and inheritable properties (like font settings, colors) that can be passed
/// down to child elements.
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[serde(default)]
#[ts(export, optional_fields)]
pub struct Style {
  /// Width of the element
  pub width: ValuePercentageAuto,
  /// Height of the element
  pub height: ValuePercentageAuto,
  /// Internal spacing around the element's content
  pub padding: SidesValue<ValuePercentageAuto>,
  /// External spacing around the element
  pub margin: SidesValue<ValuePercentageAuto>,
  /// Positioning offset from the element's normal position
  pub inset: SidesValue<ValuePercentageAuto>,
  /// Direction of flex layout (row or column)
  pub flex_direction: FlexDirection,
  /// How flex items are aligned along the main axis
  pub justify_content: Option<JustifyContent>,
  /// How flex items are aligned along the cross axis
  pub align_items: Option<AlignItems>,
  /// Positioning method (relative, absolute, etc.)
  pub position: Position,
  /// Spacing between flex items
  pub gap: Gap,
  /// How much the element should grow relative to other flex items
  pub flex_grow: f32,
  /// Width of the element's border
  pub border_width: SidesValue<ValuePercentageAuto>,
  /// How images should be fitted within their container
  pub object_fit: ObjectFit,
  /// Element's background color
  pub background_color: Option<ColorInput>,
  /// Inheritable style properties
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
      border_width: Default::default(),
      object_fit: Default::default(),
      background_color: Default::default(),
      inheritable_style: Default::default(),
    }
  }
}

/// Style properties that can be inherited by child elements.
///
/// These properties are typically passed down from parent to child elements
/// in the layout hierarchy, such as font settings and colors.
#[derive(Debug, Clone, Deserialize, Default, Serialize, Merge, TS)]
#[merge(strategy = overwrite_none)]
#[ts(optional_fields)]
pub struct InheritableStyle {
  /// Color of the element's border
  pub border_color: Option<ColorInput>,
  /// Text color for child text elements
  pub color: Option<ColorInput>,
  /// Font size in pixels for text rendering
  pub font_size: Option<f32>,
  /// Font family name for text rendering
  pub font_family: Option<String>,
  /// Line height multiplier for text spacing
  pub line_height: Option<f32>,
  /// Font weight for text rendering
  pub font_weight: Option<FontWeight>,
  /// Maximum number of lines for text before truncation
  pub max_lines: Option<u32>,
  /// Corner radius for rounded borders in pixels
  pub border_radius: Option<SidesValue<ValuePercentageAuto>>,
  /// Text alignment within the element
  pub text_align: Option<TextAlign>,
}

/// Font styling properties for text rendering.
///
/// This structure contains all the necessary information for rendering text,
/// including font size, family, weight, alignment, and color.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FontStyle {
  /// Font size in pixels
  pub font_size: f32,
  /// Font family name
  pub font_family: Option<String>,
  /// Text alignment
  pub text_align: TextAlign,
  /// Line height multiplier
  pub line_height: f32,
  /// Font weight
  pub font_weight: FontWeight,
  /// Maximum number of lines before truncation
  pub max_lines: Option<u32>,
  /// Text color
  pub color: ColorInput,
}

impl Default for FontStyle {
  fn default() -> Self {
    Self {
      font_size: 16.0,
      font_family: None,
      line_height: 1.0,
      font_weight: FontWeight::default(),
      max_lines: None,
      color: ColorInput::default(),
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
      color: style.inheritable_style.color.clone().unwrap_or_default(),
      text_align: style.inheritable_style.text_align.unwrap_or_default(),
    }
  }
}

/// Represents spacing between flex items.
///
/// Can be either a single value applied to both axes, or separate values
/// for horizontal and vertical spacing.
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[serde(untagged)]
pub enum Gap {
  /// Same gap value for both horizontal and vertical spacing
  SingleValue(f32),
  /// Separate values for horizontal and vertical spacing (horizontal, vertical)
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

/// Border styling properties.
///
/// Defines the visual appearance of element borders, including color and size.
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(optional_fields)]
pub struct Border {
  /// Border color
  pub color: Option<ColorInput>,
  /// Border size for each side
  pub size: SidesValue<f32>,
}

/// Represents values that can be applied to all sides of an element.
///
/// This enum allows for flexible specification of values like padding, margin,
/// or border sizes using either a single value for all sides, separate values
/// for vertical/horizontal axes, or individual values for each side.
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[serde(untagged)]
pub enum SidesValue<T> {
  /// Same value for all four sides
  SingleValue(T),
  /// Separate values for vertical and horizontal sides (vertical, horizontal)
  AxisSidesArray(T, T),
  /// Individual values for each side (top, right, bottom, left)
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

/// Represents a value that can be a specific length, percentage, or automatic.
///
/// This corresponds to CSS values that can be specified as pixels, percentages,
/// or the 'auto' keyword for automatic sizing.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Copy, TS)]
#[serde(rename_all = "camelCase")]
pub enum ValuePercentageAuto {
  /// Automatic sizing based on content
  Auto,
  /// Percentage value relative to parent container
  Percentage(f32),
  /// Specific pixel value
  #[serde(untagged)]
  SpecificValue(f32),
}

impl From<f32> for ValuePercentageAuto {
  fn from(value: f32) -> Self {
    Self::SpecificValue(value)
  }
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

impl From<Style> for TaffyStyle {
  fn from(style: Style) -> Self {
    Self {
      size: Size {
        width: style.width.into(),
        height: style.height.into(),
      },
      border: style.border_width.into(),
      padding: style.padding.into(),
      inset: style.inset.into(),
      margin: style.margin.into(),
      display: Display::Flex,
      flex_direction: style.flex_direction.into(),
      position: style.position.into(),
      justify_content: style.justify_content.map(|j| j.into()),
      flex_grow: style.flex_grow,
      align_items: style.align_items.map(|a| a.into()),
      gap: style.gap.into(),
      ..Default::default()
    }
  }
}
