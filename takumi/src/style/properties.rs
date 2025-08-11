//! Style properties and related types for the takumi styling system.
//!
//! This module contains CSS-like properties including layout properties,
//! typography settings, positioning, and visual effects.

use cosmic_text::{Align, FamilyOwned, Weight};
use image::imageops::FilterType;
use merge::{Merge, option::overwrite_none};
use serde::{Deserialize, Serialize};
use taffy::{
  CompactLength, GridTemplateRepetition, MaxTrackSizingFunction, MinTrackSizingFunction, Size,
  Style as TaffyStyle, TrackSizingFunction,
};
use ts_rs::TS;

use crate::{
  Gradient,
  core::{DEFAULT_FONT_SIZE, DEFAULT_LINE_HEIGHT_SCALER, viewport::RenderContext},
  impl_from_taffy_enum,
  style::{
    Color, ColorInput, Gap, LengthUnit, SidesValue, resolve_length_unit_rect_to_length_percentage,
    resolve_length_unit_rect_to_length_percentage_auto,
  },
};

/// Represents font weight as a numeric value.
///
/// This wraps a u16 value that corresponds to CSS font-weight values.
/// Common values include 100 (thin), 200 (extra light), 300 (light),
/// 400 (normal), 500 (medium), 600 (semi bold), 700 (bold),
/// 800 (extra bold), 900 (black).
#[derive(Debug, Copy, Clone, Deserialize, Serialize, TS, PartialEq)]
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
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ObjectFit {
  /// The replaced content is sized to fill the element's content box exactly, without maintaining aspect ratio
  #[default]
  Fill,
  /// The replaced content is scaled to maintain its aspect ratio while fitting within the element's content box
  Contain,
  /// The replaced content is sized to maintain its aspect ratio while filling the element's entire content box
  Cover,
  /// The content is sized as if none or contain were specified, whichever would result in a smaller concrete object size
  ScaleDown,
  /// The replaced content is not resized and maintains its intrinsic dimensions
  None,
}

/// Text alignment options for text rendering.
///
/// Corresponds to CSS text-align property values.
#[derive(Default, Debug, Clone, Deserialize, Serialize, Copy, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TextAlign {
  /// Aligns inline content to the left edge of the line box
  Left,
  /// Aligns inline content to the right edge of the line box
  Right,
  /// Centers inline content within the line box
  Center,
  /// Expands inline content to fill the entire line box
  Justify,
  /// Aligns inline content to the start edge of the line box (language-dependent)
  #[default]
  Start,
  /// Aligns inline content to the end edge of the line box (language-dependent)
  End,
}

impl From<TextAlign> for Option<Align> {
  fn from(value: TextAlign) -> Self {
    match value {
      TextAlign::Left => Some(Align::Left),
      TextAlign::Right => Some(Align::Right),
      TextAlign::Center => Some(Align::Center),
      TextAlign::Justify => Some(Align::Justified),
      TextAlign::End => Some(Align::End),
      TextAlign::Start => None,
    }
  }
}

/// Defines the positioning method for an element.
///
/// This enum determines how an element is positioned within its containing element.
#[derive(Default, Debug, Clone, Deserialize, Serialize, Copy, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Position {
  /// The element is positioned according to the normal flow of the document.
  /// Offsets (top, right, bottom, left) have no effect.
  #[default]
  Relative,
  /// The element is removed from the normal document flow and positioned relative to its nearest positioned ancestor.
  /// Offsets (top, right, bottom, left) specify the distance from the ancestor.
  Absolute,
}

impl_from_taffy_enum!(Position, taffy::Position, Relative, Absolute);

/// Defines the direction of flex items within a flex container.
///
/// This enum determines how flex items are laid out along the main axis.
#[derive(Default, Debug, Clone, Deserialize, Serialize, Copy, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum FlexDirection {
  /// Items are laid out in the same direction as the text direction (left-to-right for English)
  #[default]
  Row,
  /// Items are laid out perpendicular to the text direction (top-to-bottom)
  Column,
  /// Items are laid out in the opposite direction to the text direction (right-to-left for English)
  RowReverse,
  /// Items are laid out opposite to the column direction (bottom-to-top)
  ColumnReverse,
}

impl_from_taffy_enum!(
  FlexDirection,
  taffy::FlexDirection,
  Row,
  Column,
  RowReverse,
  ColumnReverse
);

/// Defines a box shadow for an element.
///
/// This struct contains the properties for a box shadow, including color,
/// offset, blur radius, spread radius, and inset flag.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
pub struct BoxShadow {
  /// Color of the box shadow
  pub color: ColorInput,
  /// Horizontal offset of the box shadow
  pub offset_x: LengthUnit,
  /// Vertical offset of the box shadow
  pub offset_y: LengthUnit,
  /// Blur radius of the box shadow (must be non-negative)
  pub blur_radius: LengthUnit,
  /// Spread radius of the box shadow (can be negative)
  pub spread_radius: LengthUnit,
  /// Whether the shadow is inset (inside the element) or outset (outside the element)
  #[serde(default)]
  pub inset: bool,
}

impl BoxShadow {
  pub(crate) fn resolve(self, context: &RenderContext) -> BoxShadowResolved {
    BoxShadowResolved {
      color: self.color,
      offset_x: self.offset_x.resolve_to_px(context),
      offset_y: self.offset_y.resolve_to_px(context),
      blur_radius: self.blur_radius.resolve_to_px(context),
      spread_radius: self.spread_radius.resolve_to_px(context),
      inset: self.inset,
    }
  }
}

/// Represents a resolved box shadow with concrete pixel values.
///
/// This struct contains the final computed values for a box shadow
/// after resolving relative units to absolute pixels.
pub(crate) struct BoxShadowResolved {
  /// Color of the box shadow
  pub color: ColorInput,
  /// Horizontal offset in pixels
  pub offset_x: f32,
  /// Vertical offset in pixels
  pub offset_y: f32,
  /// Blur radius in pixels
  pub blur_radius: f32,
  /// Spread radius in pixels
  pub spread_radius: f32,
  /// Whether the shadow is inset (inside the element)
  pub inset: bool,
}

/// Defines a box shadow for an element.
///
/// This enum allows for flexible specification of box shadows, including
/// a single shadow or multiple shadows.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(untagged)]
pub enum BoxShadowInput {
  /// A single box shadow
  Single(BoxShadow),
  /// Multiple box shadows
  Multiple(Vec<BoxShadow>),
}

/// Defines how flex items are aligned along the main axis.
///
/// This enum determines how space is distributed between and around flex items
/// along the main axis of the flex container.
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum JustifyContent {
  /// Items are packed toward the start of the line.
  Start,
  /// Items are packed toward the end of the line.
  End,
  /// Items are packed toward the flex container's main-start side.
  /// For flex containers with flex_direction RowReverse or ColumnReverse, this is equivalent
  /// to End. In all other cases it is equivalent to Start.
  FlexStart,
  /// Items are packed toward the flex container's main-end side.
  /// For flex containers with flex_direction RowReverse or ColumnReverse, this is equivalent
  /// to Start. In all other cases it is equivalent to End.
  FlexEnd,
  /// Items are packed toward the center of the line.
  Center,
  /// Items are stretched to fill the container (only applies to flex containers)
  Stretch,
  /// Items are evenly distributed in the line; first item is on the start line,
  /// last item on the end line.
  SpaceBetween,
  /// Items are evenly distributed in the line with equal space around them.
  SpaceEvenly,
  /// Items are evenly distributed in the line; first item is on the start line,
  /// last item on the end line, and the space between items is twice the space
  /// between the start/end items and the container edges.
  SpaceAround,
}

impl_from_taffy_enum!(
  JustifyContent,
  taffy::JustifyContent,
  Start,
  End,
  FlexStart,
  FlexEnd,
  Center,
  Stretch,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly
);

/// This enum determines the layout algorithm used for the children of a node.
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Display {
  /// The element generates a flex container and its children follow the flexbox layout algorithm
  Flex,
  /// The element generates a grid container and its children follow the CSS Grid layout algorithm
  Grid,
}

impl_from_taffy_enum!(Display, taffy::Display, Flex, Grid);

/// Defines how flex items are aligned along the cross axis.
///
/// This enum determines how flex items are aligned within the flex container
/// along the cross axis (perpendicular to the main axis).
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum AlignItems {
  /// Items are aligned to the start of the line in the cross axis
  Start,
  /// Items are aligned to the end of the line in the cross axis
  End,
  /// Items are aligned to the flex container's cross-start side
  FlexStart,
  /// Items are aligned to the flex container's cross-end side
  FlexEnd,
  /// Items are centered in the cross axis
  Center,
  /// Items are aligned so that their baselines align
  Baseline,
  /// Items are stretched to fill the container in the cross axis
  Stretch,
}

impl_from_taffy_enum!(
  AlignItems,
  taffy::AlignItems,
  Start,
  End,
  FlexStart,
  FlexEnd,
  Center,
  Baseline,
  Stretch
);

/// Defines how flex items should wrap.
///
/// This enum determines how flex items should wrap within the flex container.
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum FlexWrap {
  /// Flex items will all be displayed in a single line, shrinking as needed
  #[serde(rename = "nowrap")]
  NoWrap,
  /// Flex items will wrap onto multiple lines, with new lines stacking in the flex direction
  Wrap,
  /// Flex items will wrap onto multiple lines, with new lines stacking in the reverse flex direction
  WrapReverse,
}

impl_from_taffy_enum!(FlexWrap, taffy::FlexWrap, NoWrap, Wrap, WrapReverse);

/// Defines how text should be overflowed.
///
/// This enum determines how text should be handled when it exceeds the container width.
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TextOverflow {
  /// Text is truncated with an ellipsis (…) at the end when it overflows
  Ellipsis,
  /// Text is simply clipped at the overflow edge with no visual indication
  Clip,
}

/// Represents a font family for text rendering.
/// Use only the family name (no style suffixes like "Bold", "Italic", "Regular").
/// Multi-word names are allowed (e.g., "Noto Sans") and should be provided as-is without quotes.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum FontFamily {
  /// CSS 'sans-serif' generic family
  SansSerif,
  /// CSS 'serif' generic family
  Serif,
  /// CSS 'monospace' generic family
  Monospace,
  /// CSS 'cursive' generic family
  Cursive,
  /// CSS 'fantasy' generic family
  Fantasy,
  /// A specific font family name, no suffixes like "Bold", "Italic", etc.
  #[serde(untagged)]
  Custom(String),
}

impl From<FontFamily> for FamilyOwned {
  fn from(family: FontFamily) -> Self {
    match family {
      FontFamily::SansSerif => FamilyOwned::SansSerif,
      FontFamily::Serif => FamilyOwned::Serif,
      FontFamily::Monospace => FamilyOwned::Monospace,
      FontFamily::Cursive => FamilyOwned::Cursive,
      FontFamily::Fantasy => FamilyOwned::Fantasy,
      FontFamily::Custom(name) => FamilyOwned::Name(name.into()),
    }
  }
}

/// Represents the resolved font style for a text node.
///
/// This struct contains the resolved font style properties after inheriting
/// from parent elements and converting relative units to absolute values.
#[derive(Debug, Clone)]
pub struct ResolvedFontStyle {
  /// Font size in pixels for text rendering
  pub font_size: f32,
  /// Line height as an absolute value in pixels
  pub line_height: f32,
  /// Font weight for text rendering
  pub font_weight: Weight,
  /// Maximum number of lines for text before truncation
  pub line_clamp: Option<u32>,
  /// Font family name for text rendering
  pub font_family: Option<FamilyOwned>,
  /// Letter spacing for text rendering in em units (relative to font size)
  pub letter_spacing: Option<f32>,
  /// Text alignment within the element
  pub text_align: Option<Align>,
  /// How text should be overflowed
  pub text_overflow: TextOverflow,
  /// Text color for child text elements
  pub color: ColorInput,
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
  /// Display algorithm to use for the element
  pub display: Display,
  /// Width of the element
  pub width: LengthUnit,
  /// Height of the element
  pub height: LengthUnit,
  /// Max width of the element
  pub max_width: LengthUnit,
  /// Max height of the element
  pub max_height: LengthUnit,
  /// Min width of the element
  pub min_width: LengthUnit,
  /// Min height of the element
  pub min_height: LengthUnit,
  /// Aspect ratio of the element (width/height)
  pub aspect_ratio: Option<f32>,
  /// Internal spacing around the element's content (top, right, bottom, left)
  pub padding: SidesValue<LengthUnit>,
  /// External spacing around the element (top, right, bottom, left)
  pub margin: SidesValue<LengthUnit>,
  /// Positioning offsets (top, right, bottom, left) from the element's normal position
  pub inset: SidesValue<LengthUnit>,
  /// Direction of flex layout (row or column)
  pub flex_direction: FlexDirection,
  /// How a single grid item is aligned along the inline (row) axis, overriding the container's justify-items value
  pub justify_self: Option<AlignItems>,
  /// How items are aligned along the main axis
  pub justify_content: Option<JustifyContent>,
  /// How lines are aligned within the flex container when there's extra space in the cross axis
  pub align_content: Option<JustifyContent>,
  /// How grid items are aligned along the inline (row) axis within their grid areas
  pub justify_items: Option<AlignItems>,
  /// How items are aligned along the cross axis
  pub align_items: Option<AlignItems>,
  /// How a single item is aligned along the cross axis, overriding the container's align-items value
  pub align_self: Option<AlignItems>,
  /// How flex items should wrap
  pub flex_wrap: FlexWrap,
  /// The initial main size of the flex item before growing or shrinking
  pub flex_basis: LengthUnit,
  /// Positioning method (relative, absolute, etc.)
  pub position: Position,
  /// Spacing between rows and columns in flex or grid layouts
  pub gap: Gap,
  /// How much the flex item should grow relative to other flex items when positive free space is distributed
  pub flex_grow: f32,
  /// How much the flex item should shrink relative to other flex items when negative free space is distributed
  pub flex_shrink: f32,
  /// Width of the element's border on each side (top, right, bottom, left)
  pub border_width: SidesValue<LengthUnit>,
  /// How images should be fitted within their container
  pub object_fit: ObjectFit,
  /// Background gradient(s)
  pub background_image: Option<Gradient>,
  /// Background color for the element
  pub background_color: Option<Color>,
  /// Box shadow effect for the element
  pub box_shadow: Option<BoxShadowInput>,
  /// Controls the size of implicitly-created grid columns
  pub grid_auto_columns: Option<Vec<GridTrackSize>>,
  /// Controls the size of implicitly-created grid rows
  pub grid_auto_rows: Option<Vec<GridTrackSize>>,
  /// Controls how auto-placed items are inserted in the grid
  pub grid_auto_flow: Option<GridAutoFlow>,
  /// Specifies a grid item's size and location within the grid column
  pub grid_column: Option<GridLine>,
  /// Specifies a grid item's size and location within the grid row
  pub grid_row: Option<GridLine>,
  /// Defines the line names and track sizing functions of the grid columns
  pub grid_template_columns: Option<Vec<GridTemplateComponent>>,
  /// Defines the line names and track sizing functions of the grid rows
  pub grid_template_rows: Option<Vec<GridTemplateComponent>>,
  /// Inheritable style properties that cascade to child elements
  #[serde(flatten)]
  pub inheritable_style: InheritableStyle,
}

impl Default for Style {
  fn default() -> Self {
    Self {
      display: Display::Flex,
      margin: SidesValue::SingleValue(LengthUnit::Px(0.0)),
      padding: SidesValue::SingleValue(LengthUnit::Px(0.0)),
      width: Default::default(),
      height: Default::default(),
      max_width: Default::default(),
      max_height: Default::default(),
      min_width: Default::default(),
      min_height: Default::default(),
      aspect_ratio: None,
      inset: Default::default(),
      flex_direction: Default::default(),
      justify_content: Default::default(),
      align_content: Default::default(),
      justify_self: Default::default(),
      justify_items: Default::default(),
      align_items: Default::default(),
      align_self: Default::default(),
      position: Default::default(),
      gap: Default::default(),
      flex_grow: 0.0,
      flex_shrink: 1.0,
      flex_basis: Default::default(),
      flex_wrap: FlexWrap::NoWrap,
      border_width: SidesValue::SingleValue(LengthUnit::Px(0.0)),
      object_fit: Default::default(),
      box_shadow: Default::default(),
      background_color: None,
      background_image: None,
      grid_auto_columns: None,
      grid_auto_rows: None,
      grid_auto_flow: None,
      grid_column: None,
      grid_row: None,
      grid_template_columns: None,
      grid_template_rows: None,
      inheritable_style: Default::default(),
    }
  }
}

/// Represents a grid track sizing function with serde support
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GridLengthUnit {
  /// A fraction of the available space
  Fr(f32),
  /// A fixed length
  #[serde(untagged)]
  Unit(LengthUnit),
}

/// Represents a grid minmax()
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
pub struct GridMinMaxSize {
  /// The minimum size of the grid item
  pub min: GridLengthUnit,
  /// The maximum size of the grid item
  pub max: GridLengthUnit,
}

/// Represents a grid track size
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(untagged)]
pub enum GridTrackSize {
  /// A minmax() track size
  MinMax(GridMinMaxSize),
  /// A fixed track size
  Fixed(GridLengthUnit),
}

impl From<GridLengthUnit> for GridTrackSize {
  fn from(length: GridLengthUnit) -> Self {
    Self::Fixed(length)
  }
}

/// Represents a grid repeat track
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
pub struct GridRepeatTrack {
  /// The size of the grid track
  pub size: GridTrackSize,
  /// The names of the grid lines
  pub names: Vec<String>,
}

impl GridLengthUnit {
  /// Converts the grid track size to a compact length representation.
  pub fn to_compact_length(&self, context: &RenderContext) -> CompactLength {
    match self {
      GridLengthUnit::Fr(fr) => CompactLength::fr(*fr),
      GridLengthUnit::Unit(unit) => unit.to_compact_length(context),
    }
  }
}

impl GridTrackSize {
  /// Converts the grid track size to a non-repeated track sizing function.
  pub fn to_min_max(&self, context: &RenderContext) -> TrackSizingFunction {
    match self {
      // SAFETY: The compact length is a valid track sizing function.
      Self::Fixed(size) => unsafe {
        TrackSizingFunction {
          min: MinTrackSizingFunction::from_raw(size.to_compact_length(context)),
          max: MaxTrackSizingFunction::from_raw(size.to_compact_length(context)),
        }
      },
      Self::MinMax(min_max) => unsafe {
        TrackSizingFunction {
          min: MinTrackSizingFunction::from_raw(min_max.min.to_compact_length(context)),
          max: MaxTrackSizingFunction::from_raw(min_max.max.to_compact_length(context)),
        }
      },
    }
  }
}

/// Represents a grid line placement with serde support
#[derive(Debug, Clone, Deserialize, Serialize, TS, Default, PartialEq)]
pub struct GridLine {
  /// The start line placement
  pub start: Option<GridPlacement>,
  /// The end line placement
  pub end: Option<GridPlacement>,
}

impl From<GridLine> for taffy::Line<taffy::GridPlacement> {
  fn from(line: GridLine) -> Self {
    Self {
      start: line.start.unwrap_or_default().into(),
      end: line.end.unwrap_or_default().into(),
    }
  }
}

/// Represents a grid placement with serde support
#[derive(Debug, Clone, Deserialize, Serialize, TS, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GridPlacement {
  /// Auto placement
  #[default]
  Auto,
  /// Span count
  Span(u16),
  /// Line index (1-based)
  #[serde(untagged)]
  Line(i16),
  #[serde(untagged)]
  /// Named grid area
  Named(String),
}

// Note: GridPlacement has a custom conversion due to its complex nature
impl From<GridPlacement> for taffy::GridPlacement {
  fn from(placement: GridPlacement) -> Self {
    match placement {
      GridPlacement::Auto => taffy::GridPlacement::Auto,
      GridPlacement::Line(line) => taffy::GridPlacement::Line(line.into()),
      GridPlacement::Span(span) => taffy::GridPlacement::Span(span),
      GridPlacement::Named(_) => taffy::GridPlacement::Auto,
    }
  }
}

/// Represents a grid track repetition pattern
#[derive(Debug, Clone, Deserialize, Serialize, TS, Copy, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GridRepetitionCount {
  /// Automatically fills the available space with as many tracks as possible
  AutoFill,
  /// Automatically fits as many tracks as possible while maintaining minimum size
  AutoFit,
  /// Specifies an exact number of track repetitions
  #[serde(untagged)]
  Count(u16),
}

impl From<GridRepetitionCount> for taffy::RepetitionCount {
  fn from(repetition: GridRepetitionCount) -> Self {
    match repetition {
      GridRepetitionCount::AutoFill => taffy::RepetitionCount::AutoFill,
      GridRepetitionCount::AutoFit => taffy::RepetitionCount::AutoFit,
      GridRepetitionCount::Count(count) => taffy::RepetitionCount::Count(count),
    }
  }
}

/// Represents a track sizing function
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GridTemplateComponent {
  /// A single non-repeated track
  Single(GridTrackSize),
  /// Automatically generate grid tracks to fit the available space using the specified definite track lengths
  /// Only valid if every track in template (not just the repetition) has a fixed size.
  Repeat(GridRepetitionCount, Vec<GridRepeatTrack>),
}

impl GridTemplateComponent {
  /// Converts this track sizing function to a Taffy-compatible format.
  ///
  /// # Arguments
  ///
  /// * `context` - The render context containing viewport information for unit resolution
  ///
  /// # Returns
  ///
  /// A `taffy::GridTemplateComponent` that can be used with the Taffy layout engine
  fn to_taffy(&self, context: &RenderContext) -> taffy::GridTemplateComponent<String> {
    match self {
      Self::Single(track_size) => {
        taffy::GridTemplateComponent::Single(track_size.to_min_max(context))
      }
      Self::Repeat(repetition, tracks) => {
        let track_sizes = tracks
          .iter()
          .map(|track| track.size.to_min_max(context))
          .collect();
        let line_names = tracks.iter().map(|track| track.names.clone()).collect();

        taffy::GridTemplateComponent::Repeat(GridTemplateRepetition {
          count: (*repetition).into(),
          tracks: track_sizes,
          line_names,
        })
      }
    }
  }
}

/// Represents the grid auto flow with serde support
#[derive(Debug, Clone, Copy, Deserialize, Serialize, TS, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GridAutoFlow {
  /// Places grid items by filling each row in turn, adding new rows as needed
  #[default]
  Row,
  /// Places grid items by filling each column in turn, adding new columns as needed
  Column,
  /// Places grid items by filling each row in turn, using dense packing to fill gaps
  RowDense,
  /// Places grid items by filling each column in turn, using dense packing to fill gaps
  ColumnDense,
}

impl_from_taffy_enum!(
  GridAutoFlow,
  taffy::style::GridAutoFlow,
  Row,
  Column,
  RowDense,
  ColumnDense
);

/// Defines how images should be scaled when rendered.
#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ImageScalingAlgorithm {
  /// The image is scaled using Catmull-Rom interpolation.
  /// This is balanced for speed and quality.
  #[default]
  Auto,
  /// The image is scaled using Lanczos3 resampling.
  /// This provides high-quality scaling but may be slower.
  Smooth,
  /// The image is scaled using nearest neighbor interpolation,
  /// which is suitable for pixel art or images where sharp edges are desired.
  Pixelated,
}

impl From<ImageScalingAlgorithm> for FilterType {
  fn from(algorithm: ImageScalingAlgorithm) -> Self {
    match algorithm {
      ImageScalingAlgorithm::Auto => FilterType::CatmullRom,
      ImageScalingAlgorithm::Smooth => FilterType::Lanczos3,
      ImageScalingAlgorithm::Pixelated => FilterType::Nearest,
    }
  }
}

/// Style properties that can be inherited by child elements.
///
/// These properties are typically passed down from parent to child elements
/// in the layout hierarchy, such as font settings and colors.
#[derive(Debug, Clone, Deserialize, Default, Serialize, Merge, TS)]
#[merge(strategy = overwrite_none)]
#[ts(optional_fields, export)]
pub struct InheritableStyle {
  /// How text should be overflowed
  pub text_overflow: Option<TextOverflow>,
  /// Color of the element's border
  pub border_color: Option<ColorInput>,
  /// Text color for child text elements
  pub color: Option<ColorInput>,
  /// Font size in pixels for text rendering
  pub font_size: Option<LengthUnit>,
  /// Font family name for text rendering
  pub font_family: Option<FontFamily>,
  /// Line height multiplier for text spacing
  pub line_height: Option<LengthUnit>,
  /// Font weight for text rendering
  pub font_weight: Option<FontWeight>,
  /// Maximum number of lines for text before truncation
  pub line_clamp: Option<u32>,
  /// Corner radius for rounded borders
  pub border_radius: Option<SidesValue<LengthUnit>>,
  /// Text alignment within the element
  pub text_align: Option<TextAlign>,
  /// Additional spacing between characters in text
  /// Positive values increase spacing, negative values decrease spacing
  pub letter_spacing: Option<LengthUnit>,
  /// Controls how images are scaled when rendered
  /// This property determines the algorithm used for image scaling
  pub image_rendering: Option<ImageScalingAlgorithm>,
}

impl Style {
  /// Resolves the style to a `TaffyStyle`.
  /// Converts this style to a Taffy-compatible style for layout calculations.
  ///
  /// # Arguments
  ///
  /// * `context` - The render context containing viewport information for unit resolution
  ///
  /// # Returns
  ///
  /// A `TaffyStyle` that can be used with the Taffy layout engine
  pub fn resolve_to_taffy_style(&self, context: &RenderContext) -> TaffyStyle {
    TaffyStyle {
      size: Size {
        width: self.width.resolve_to_dimension(context),
        height: self.height.resolve_to_dimension(context),
      },
      border: resolve_length_unit_rect_to_length_percentage(context, self.border_width.into()),
      padding: resolve_length_unit_rect_to_length_percentage(context, self.padding.into()),
      inset: resolve_length_unit_rect_to_length_percentage_auto(context, self.inset.into()),
      margin: resolve_length_unit_rect_to_length_percentage_auto(context, self.margin.into()),
      display: self.display.into(),
      flex_direction: self.flex_direction.into(),
      position: self.position.into(),
      justify_content: self.justify_content.map(Into::into),
      align_content: self.align_content.map(Into::into),
      justify_items: self.justify_items.map(Into::into),
      flex_grow: self.flex_grow,
      align_items: self.align_items.map(Into::into),
      gap: self.gap.resolve_to_size(context),
      flex_basis: self.flex_basis.resolve_to_dimension(context),
      flex_shrink: self.flex_shrink,
      flex_wrap: self.flex_wrap.into(),
      min_size: Size {
        width: self.min_width.resolve_to_dimension(context),
        height: self.min_height.resolve_to_dimension(context),
      },
      max_size: Size {
        width: self.max_width.resolve_to_dimension(context),
        height: self.max_height.resolve_to_dimension(context),
      },
      // Convert grid properties to Taffy types using only public API
      grid_auto_columns: self.grid_auto_columns.as_ref().map_or_else(Vec::new, |v| {
        v.iter().map(|s| s.to_min_max(context)).collect()
      }),
      grid_auto_rows: self.grid_auto_rows.as_ref().map_or_else(Vec::new, |v| {
        v.iter().map(|s| s.to_min_max(context)).collect()
      }),
      grid_auto_flow: self.grid_auto_flow.unwrap_or_default().into(),
      grid_column: self
        .grid_column
        .as_ref()
        .map_or_else(Default::default, |line| line.clone().into()),
      grid_row: self
        .grid_row
        .as_ref()
        .map_or_else(Default::default, |line| line.clone().into()),
      grid_template_columns: self
        .grid_template_columns
        .as_ref()
        .map_or_else(Vec::new, |v| {
          v.iter().map(|s| s.to_taffy(context)).collect()
        }),
      grid_template_rows: self.grid_template_rows.as_ref().map_or_else(Vec::new, |v| {
        v.iter().map(|s| s.to_taffy(context)).collect()
      }),
      aspect_ratio: self.aspect_ratio,
      align_self: self.align_self.map(Into::into),
      justify_self: self.justify_self.map(Into::into),
      ..Default::default()
    }
  }

  /// Resolves the style to a `ResolvedFontStyle`.
  /// Resolves inheritable style properties to concrete values for text rendering.
  ///
  /// This method combines the element's inheritable styles with default values
  /// to produce a complete font style specification.
  ///
  /// # Arguments
  ///
  /// * `context` - The render context containing viewport information for unit resolution
  ///
  /// # Returns
  ///
  /// A `ResolvedFontStyle` with all font-related properties resolved to concrete values
  pub fn resolve_to_font_style(&self, context: &RenderContext) -> ResolvedFontStyle {
    let font_size = self
      .inheritable_style
      .font_size
      .map(|f| f.resolve_to_px(context))
      .unwrap_or(DEFAULT_FONT_SIZE);

    let line_height = self
      .inheritable_style
      .line_height
      .map(|f| f.resolve_to_px(context))
      .unwrap_or_else(|| font_size * DEFAULT_LINE_HEIGHT_SCALER);

    ResolvedFontStyle {
      color: self.inheritable_style.color.clone().unwrap_or_default(),
      font_size,
      line_height,
      font_weight: self
        .inheritable_style
        .font_weight
        .unwrap_or_default()
        .into(),
      line_clamp: self.inheritable_style.line_clamp,
      font_family: self.inheritable_style.font_family.clone().map(Into::into),
      letter_spacing: self
        .inheritable_style
        .letter_spacing
        .map(|spacing| spacing.resolve_to_px(context) / context.parent_font_size),
      text_align: self.inheritable_style.text_align.and_then(Into::into),
      text_overflow: self
        .inheritable_style
        .text_overflow
        .unwrap_or(TextOverflow::Clip),
    }
  }
}
