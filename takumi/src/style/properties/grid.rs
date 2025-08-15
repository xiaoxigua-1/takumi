use cssparser::{BasicParseError, ParseError, Parser, ParserInput, Token};
use serde::{Deserialize, Serialize};
use taffy::{
  CompactLength, GridTemplateRepetition, MaxTrackSizingFunction, MinTrackSizingFunction,
  TrackSizingFunction,
};
use ts_rs::TS;

use crate::{FromCss, LengthUnit, core::viewport::RenderContext};

type ParseResult<'i, T, E = BasicParseError<'i>> = Result<T, ParseError<'i, E>>;

/// Represents a grid track sizing function with serde support
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(try_from = "GridLengthUnitValue")]
#[ts(as = "GridLengthUnitValue")]
pub enum GridLengthUnit {
  /// A fraction of the available space
  Fr(f32),
  /// A fixed length
  #[serde(untagged)]
  Unit(LengthUnit),
}

/// Represents a grid length unit value with serde support
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GridLengthUnitValue {
  /// A fraction of the available space
  Fr(f32),
  /// A fixed length
  #[serde(untagged)]
  Unit(LengthUnit),
  /// A CSS string representation
  #[serde(untagged)]
  Css(String),
}

impl TryFrom<GridLengthUnitValue> for GridLengthUnit {
  type Error = &'static str;

  fn try_from(value: GridLengthUnitValue) -> Result<Self, Self::Error> {
    match value {
      GridLengthUnitValue::Fr(fr) => Ok(GridLengthUnit::Fr(fr)),
      GridLengthUnitValue::Unit(unit) => Ok(GridLengthUnit::Unit(unit)),
      GridLengthUnitValue::Css(css) => {
        let mut input = ParserInput::new(&css);
        let mut parser = Parser::new(&mut input);
        GridLengthUnit::from_css(&mut parser).map_err(|_| "Failed to parse CSS grid length unit")
      }
    }
  }
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

// Minimal CSS parsing helpers for grid values (mirror patterns used in other property modules)
impl<'i> FromCss<'i> for GridLengthUnit {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    if let Ok(unit) = LengthUnit::from_css(input) {
      return Ok(GridLengthUnit::Unit(unit));
    }

    let location = input.current_source_location();
    let token = input.next()?;

    let Token::Dimension { value, unit, .. } = &token else {
      return Err(
        location
          .new_basic_unexpected_token_error(token.clone())
          .into(),
      );
    };

    if !unit.eq_ignore_ascii_case("fr") {
      return Err(
        location
          .new_basic_unexpected_token_error(token.clone())
          .into(),
      );
    }

    Ok(GridLengthUnit::Fr(*value))
  }
}
