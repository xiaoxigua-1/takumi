//! Style properties and related types for the takumi styling system.
//!
//! This module contains CSS-like properties including layout properties,
//! typography settings, positioning, and visual effects.

/// Box shadow properties for elements.
pub mod box_shadow;
/// Color parsing and representation for styling.
pub mod color;
/// Gap properties for flex and grid layouts.
pub mod gap;
/// Length units and measurement types for the takumi styling system.
pub mod length_unit;
/// Linear gradient properties for elements.
pub mod linear_gradient;
/// Parsing utilities for style properties.
pub mod parser;
/// Sides and related utilities for specifying padding, margin, and borders.
pub mod sides;

pub use box_shadow::*;
pub use color::*;
pub use gap::*;
pub use length_unit::*;
pub use linear_gradient::*;
pub use parser::*;
pub use sides::*;

use cosmic_text::{Align, FamilyOwned, Weight};
use cssparser::{BasicParseError, ParseError, Parser};
use image::imageops::FilterType;
use serde::{Deserialize, Serialize};
use taffy::{
  CompactLength, GridTemplateRepetition, MaxTrackSizingFunction, MinTrackSizingFunction,
  TrackSizingFunction,
};
use ts_rs::TS;

use crate::{core::viewport::RenderContext, impl_from_taffy_enum};

type ParseResult<'i, T, E = BasicParseError<'i>> = Result<T, ParseError<'i, E>>;

/// Trait for types that can be deserialized from CSS.
pub trait FromCss<'i> {
  /// Deserializes the type from a CSS string.
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self>
  where
    Self: Sized;
}

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
/// This enum determines how items are aligned within the flex container
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
  pub fn to_taffy(&self, context: &RenderContext) -> taffy::GridTemplateComponent<String> {
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

// Style, InheritableStyle, and FontStyle moved to `style/stylesheets.rs`
