use std::collections::HashMap;

use cssparser::{BasicParseError, ParseError, Parser, ParserInput, Token};
use serde::{Deserialize, Serialize};
use taffy::{
  CompactLength, GridTemplateRepetition, MaxTrackSizingFunction, MinTrackSizingFunction,
  TrackSizingFunction, prelude::TaffyZero,
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
  /// The names for the line preceding this track within the repeat() clause
  pub names: Vec<String>,
  /// The names for the final line after the last track within the repeat() clause
  /// Only set on the last track of the repeat fragment. For other tracks this is None.
  pub end_names: Option<Vec<String>>,
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

/// Represents a track sizing function or a list of line names between tracks
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GridTemplateComponent {
  /// A list of line names that apply to the current grid line (e.g., [a b])
  LineNames(Vec<String>),
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
      Self::LineNames(_) => {
        // Line name entries are not converted to Taffy track components
        // Consumers should filter these out when constructing template tracks
        // and instead feed them into grid_template_*_names on the Taffy style.
        // As this method is only meaningful for track components, treat this as unreachable.
        // However, to preserve a total function, return a zero-sized track which should not be used.
        taffy::GridTemplateComponent::Single(taffy::TrackSizingFunction::ZERO)
      }
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

/// Represents `grid-template-areas` value
///
/// Supports either a 2D matrix of area names (use "." for empty) or a CSS string value
/// like: "a a ." "b b c"
#[derive(Default, Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(try_from = "GridTemplateAreasValue")]
#[ts(as = "GridTemplateAreasValue")]
pub struct GridTemplateAreas(pub Vec<Vec<String>>);

/// Serde helper that accepts either a matrix or a CSS string
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(untagged)]
pub enum GridTemplateAreasValue {
  /// A 2D matrix representation (use "." for empty)
  #[serde(untagged)]
  Matrix(Vec<Vec<String>>),
  /// A CSS string representation
  #[serde(untagged)]
  Css(String),
}

impl TryFrom<GridTemplateAreasValue> for GridTemplateAreas {
  type Error = &'static str;

  fn try_from(value: GridTemplateAreasValue) -> Result<Self, Self::Error> {
    match value {
      GridTemplateAreasValue::Matrix(matrix) => {
        // Validate consistent row lengths
        let width = matrix.first().map_or(0, Vec::len);
        if width > 0 && matrix.iter().any(|r| r.len() != width) {
          return Err("Inconsistent row lengths in grid-template-areas matrix");
        }
        Ok(GridTemplateAreas(matrix))
      }
      GridTemplateAreasValue::Css(css) => {
        let mut input = ParserInput::new(&css);
        let mut parser = Parser::new(&mut input);
        GridTemplateAreas::from_css(&mut parser)
          .map_err(|_| "Failed to parse grid-template-areas CSS value")
      }
    }
  }
}

impl From<GridTemplateAreas> for Vec<taffy::GridTemplateArea<String>> {
  fn from(value: GridTemplateAreas) -> Self {
    if value.0.is_empty() {
      return Vec::new();
    }

    let mut bounds: HashMap<&str, (usize, usize, usize, usize)> = HashMap::new();
    for (r, row) in value.0.iter().enumerate() {
      for (c, cell) in row.iter().enumerate() {
        if cell == "." {
          continue;
        }

        let entry = bounds.entry(cell.as_str()).or_insert((r, r, c, c));
        entry.0 = entry.0.min(r);
        entry.1 = entry.1.max(r);
        entry.2 = entry.2.min(c);
        entry.3 = entry.3.max(c);
      }
    }

    let mut areas: Vec<taffy::GridTemplateArea<String>> = Vec::with_capacity(bounds.len());
    for (name, (rmin, rmax, cmin, cmax)) in bounds.into_iter() {
      areas.push(taffy::GridTemplateArea {
        name: name.to_string(),
        row_start: (rmin as u16) + 1,
        row_end: (rmax as u16) + 2,
        column_start: (cmin as u16) + 1,
        column_end: (cmax as u16) + 2,
      });
    }
    areas
  }
}

impl<'i> FromCss<'i> for GridTemplateAreas {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    let mut rows: Vec<Vec<String>> = Vec::new();

    while !input.is_exhausted() {
      let location = input.current_source_location();
      let token = input.next()?;
      match token {
        Token::QuotedString(row) => {
          let row_str: &str = row.as_ref();
          let cols: Vec<String> = row_str
            .split_whitespace()
            .map(ToString::to_string)
            .collect();
          if cols.is_empty() {
            return Err(
              location
                .new_basic_unexpected_token_error(Token::QuotedString(row.clone()))
                .into(),
            );
          }
          rows.push(cols);
        }
        Token::WhiteSpace(_) => continue,
        _ => {
          return Err(
            location
              .new_basic_unexpected_token_error(token.clone())
              .into(),
          );
        }
      }
    }

    // Validate consistent column counts across rows
    if let Some(width) = rows.first().map(Vec::len) {
      if rows.iter().any(|r| r.len() != width) {
        // Create a parse error for inconsistent row lengths
        let location = input.current_source_location();
        return Err(
          location
            .new_basic_unexpected_token_error(Token::Ident("inconsistent-rows".into()))
            .into(),
        );
      }
    }

    Ok(GridTemplateAreas(rows))
  }
}

// Minimal CSS parsing helpers for grid values (mirror patterns used in other property modules)
impl<'i> FromCss<'i> for GridLengthUnit {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    if let Ok(unit) = input.try_parse(LengthUnit::from_css) {
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

impl<'i> FromCss<'i> for GridMinMaxSize {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    input.expect_function_matching("minmax")?;
    input.parse_nested_block(|input| {
      let min = GridLengthUnit::from_css(input)?;
      input.expect_comma()?;
      let max = GridLengthUnit::from_css(input)?;
      Ok(GridMinMaxSize { min, max })
    })
  }
}

impl<'i> FromCss<'i> for GridTrackSize {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    if input
      .try_parse(|input| input.expect_function_matching("minmax"))
      .is_ok()
    {
      return input.parse_nested_block(|input| {
        let min = GridLengthUnit::from_css(input)?;
        input.expect_comma()?;
        let max = GridLengthUnit::from_css(input)?;
        Ok(GridTrackSize::MinMax(GridMinMaxSize { min, max }))
      });
    }

    let length = GridLengthUnit::from_css(input)?;
    Ok(GridTrackSize::Fixed(length))
  }
}

impl<'i> FromCss<'i> for GridRepetitionCount {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
      let ident_str = ident.as_ref();
      if ident_str.eq_ignore_ascii_case("auto-fill") {
        return Ok(GridRepetitionCount::AutoFill);
      }
      if ident_str.eq_ignore_ascii_case("auto-fit") {
        return Ok(GridRepetitionCount::AutoFit);
      }
      // If it's some other ident, treat as error
      let location = input.current_source_location();
      return Err::<Self, _>(
        location
          .new_basic_unexpected_token_error(Token::Ident(ident))
          .into(),
      );
    }

    let location = input.current_source_location();
    let token = input.next()?;
    match *token {
      Token::Number {
        int_value, value, ..
      } => {
        // Prefer integer value if provided
        let count: i64 = if let Some(iv) = int_value {
          iv as i64
        } else {
          value as i64
        };
        if count < 0 {
          return Err::<Self, _>(
            location
              .new_basic_unexpected_token_error(token.clone())
              .into(),
          );
        }
        Ok(GridRepetitionCount::Count(count as u16))
      }
      _ => Err::<Self, _>(
        location
          .new_basic_unexpected_token_error(token.clone())
          .into(),
      ),
    }
  }
}

impl<'i> FromCss<'i> for GridRepeatTrack {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    // Collect any leading line name blocks: [name1 name2]
    let mut names: Vec<String> = Vec::new();

    while input.try_parse(Parser::expect_square_bracket_block).is_ok() {
      input.parse_nested_block(|i| {
        while let Ok(name) = i.try_parse(Parser::expect_ident_cloned) {
          names.push(name.as_ref().to_owned());
        }
        Ok(())
      })?;
    }

    // Parse the track size
    let size = GridTrackSize::from_css(input)?;

    // Collect any trailing line name blocks
    while input.try_parse(Parser::expect_square_bracket_block).is_ok() {
      input.parse_nested_block(|i| {
        while let Ok(name) = i.try_parse(Parser::expect_ident_cloned) {
          names.push(name.as_ref().to_owned());
        }
        Ok(())
      })?;
    }

    Ok(GridRepeatTrack {
      size,
      names,
      end_names: None,
    })
  }
}

impl<'i> FromCss<'i> for GridTemplateComponent {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    // Line name block: [name1 name2 ...]
    if input.try_parse(Parser::expect_square_bracket_block).is_ok() {
      let mut names: Vec<String> = Vec::new();
      input.parse_nested_block(|i| {
        while let Ok(name) = i.try_parse(Parser::expect_ident_cloned) {
          names.push(name.as_ref().to_owned());
        }
        Ok(())
      })?;
      return Ok(GridTemplateComponent::LineNames(names));
    }

    if input
      .try_parse(|i| i.expect_function_matching("repeat"))
      .is_ok()
    {
      return input.parse_nested_block(|input| {
        let repetition = GridRepetitionCount::from_css(input)?;
        input.expect_comma()?;

        let mut tracks: Vec<GridRepeatTrack> = Vec::new();
        // Names encountered after a size belong to the NEXT track in repeat() context
        let mut pending_leading_names: Vec<String> = Vec::new();
        loop {
          // Start with any pending names from the previous track's trailing names
          let mut names: Vec<String> = std::mem::take(&mut pending_leading_names);

          // Capture any additional leading square-bracketed names before the size
          while input.try_parse(Parser::expect_square_bracket_block).is_ok() {
            input.parse_nested_block(|i| {
              while let Ok(name) = i.try_parse(Parser::expect_ident_cloned) {
                names.push(name.as_ref().to_owned());
              }
              Ok(())
            })?;
          }

          // If we cannot parse a size, stop the loop
          let size = if let Ok(size) = input.try_parse(GridTrackSize::from_css) {
            size
          } else {
            break;
          };

          // Collect trailing names, but assign them to the next track
          while input.try_parse(Parser::expect_square_bracket_block).is_ok() {
            input.parse_nested_block(|i| {
              while let Ok(name) = i.try_parse(Parser::expect_ident_cloned) {
                pending_leading_names.push(name.as_ref().to_owned());
              }
              Ok(())
            })?;
          }

          tracks.push(GridRepeatTrack {
            size,
            names,
            end_names: None,
          });
        }

        // Any remaining pending names after the final size are the trailing names of the repeat fragment
        if !pending_leading_names.is_empty() {
          if let Some(last) = tracks.last_mut() {
            last.end_names = Some(std::mem::take(&mut pending_leading_names));
          }
        }

        Ok(GridTemplateComponent::Repeat(repetition, tracks))
      });
    }

    // Single track-size
    let size = GridTrackSize::from_css(input)?;
    Ok(GridTemplateComponent::Single(size))
  }
}

impl<'i> FromCss<'i> for GridPlacement {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
      let ident_str = ident.as_ref();
      if ident_str.eq_ignore_ascii_case("auto") {
        return Ok(GridPlacement::Auto);
      }
      if ident_str.eq_ignore_ascii_case("span") {
        // Next token should be a number or ident
        // Try integer first
        if let Ok(count) = input.try_parse(Parser::expect_integer) {
          let count = if count <= 0 { 1 } else { count as u16 };
          return Ok(GridPlacement::Span(count));
        }

        // Try identifier span name (treated as span 1 for named; enum only carries count)
        if let Ok(_name) = input.try_parse(Parser::expect_ident_cloned) {
          return Ok(GridPlacement::Span(1));
        }

        // If neither, error
        let location = input.current_source_location();
        let token = input.next()?;
        return Err(
          location
            .new_basic_unexpected_token_error(token.clone())
            .into(),
        );
      }

      // Any other ident is a named line
      return Ok(GridPlacement::Named(ident_str.to_owned()));
    }

    // Try a line index (number, may be negative)
    let location = input.current_source_location();
    let token = input.next()?;
    match *token {
      Token::Number {
        int_value, value, ..
      } => {
        let v: i32 = int_value.unwrap_or(value as i32);
        Ok(GridPlacement::Line(v as i16))
      }
      _ => Err(
        location
          .new_basic_unexpected_token_error(token.clone())
          .into(),
      ),
    }
  }
}

impl<'i> FromCss<'i> for GridLine {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    // First placement is required
    let first = GridPlacement::from_css(input).ok();

    // Optional delimiter '/'
    let second = if input.try_parse(|i| i.expect_delim('/')).is_ok() {
      GridPlacement::from_css(input).ok()
    } else {
      None
    };

    if first.is_none() && second.is_none() {
      let location = input.current_source_location();
      let token = input.next()?;
      return Err(
        location
          .new_basic_unexpected_token_error(token.clone())
          .into(),
      );
    }

    Ok(GridLine {
      start: first,
      end: second,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cssparser::{Parser, ParserInput};

  fn parse_grid_length(input: &str) -> ParseResult<'_, GridLengthUnit> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);
    GridLengthUnit::from_css(&mut parser)
  }

  fn parse_grid_track_size(input: &str) -> ParseResult<'_, GridTrackSize> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);
    GridTrackSize::from_css(&mut parser)
  }

  fn parse_repetition(input: &str) -> ParseResult<'_, GridRepetitionCount> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);
    GridRepetitionCount::from_css(&mut parser)
  }

  fn parse_repeat_track(input: &str) -> ParseResult<'_, GridRepeatTrack> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);
    GridRepeatTrack::from_css(&mut parser)
  }

  fn parse_template_component(input: &str) -> ParseResult<'_, GridTemplateComponent> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);
    GridTemplateComponent::from_css(&mut parser)
  }

  fn parse_placement(input: &str) -> ParseResult<'_, GridPlacement> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);
    GridPlacement::from_css(&mut parser)
  }

  fn parse_line(input: &str) -> ParseResult<'_, GridLine> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);
    GridLine::from_css(&mut parser)
  }

  #[test]
  fn test_parse_fr_and_unit() {
    let fr = parse_grid_length("1fr").unwrap();
    assert_eq!(fr, GridLengthUnit::Fr(1.0));

    let px = parse_grid_length("10px").unwrap();
    assert_eq!(px, GridLengthUnit::Unit(LengthUnit::Px(10.0)));
  }

  #[test]
  fn test_parse_minmax_and_track_size() {
    let minmax = parse_grid_track_size("minmax(10px, 1fr)").unwrap();
    match minmax {
      GridTrackSize::MinMax(m) => {
        assert_eq!(m.min, GridLengthUnit::Unit(LengthUnit::Px(10.0)));
        assert_eq!(m.max, GridLengthUnit::Fr(1.0));
      }
      _ => panic!("expected minmax"),
    }

    let fixed = parse_grid_track_size("2fr").unwrap();
    assert_eq!(fixed, GridTrackSize::Fixed(GridLengthUnit::Fr(2.0)));
  }

  #[test]
  fn test_parse_repetition_count() {
    assert_eq!(
      parse_repetition("auto-fill").unwrap(),
      GridRepetitionCount::AutoFill
    );
    assert_eq!(
      parse_repetition("auto-fit").unwrap(),
      GridRepetitionCount::AutoFit
    );
    assert_eq!(
      parse_repetition("3").unwrap(),
      GridRepetitionCount::Count(3)
    );
  }

  #[test]
  fn test_parse_repeat_track_with_names() {
    let track = parse_repeat_track("[a b] 1fr [c]").unwrap();
    assert_eq!(track.size, GridTrackSize::Fixed(GridLengthUnit::Fr(1.0)));
    assert_eq!(
      track.names,
      vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
  }

  #[test]
  fn test_parse_template_component_repeat() {
    let tpl = parse_template_component("repeat(auto-fill, [a] 1fr [b] 2fr)").unwrap();
    match tpl {
      GridTemplateComponent::Repeat(repetition, tracks) => {
        assert_eq!(repetition, GridRepetitionCount::AutoFill);
        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].names, vec!["a".to_string()]);
        assert_eq!(tracks[1].names, vec!["b".to_string()]);
      }
      _ => panic!("expected repeat template"),
    }
  }

  #[test]
  fn test_parse_placement_and_line() {
    assert_eq!(parse_placement("auto").unwrap(), GridPlacement::Auto);
    assert_eq!(parse_placement("span 2").unwrap(), GridPlacement::Span(2));
    assert_eq!(
      parse_placement("span name").unwrap(),
      GridPlacement::Span(1)
    );
    assert_eq!(parse_placement("3").unwrap(), GridPlacement::Line(3));
    assert_eq!(parse_placement("-1").unwrap(), GridPlacement::Line(-1));
    assert_eq!(
      parse_placement("header").unwrap(),
      GridPlacement::Named("header".to_string())
    );

    let line = parse_line("span 2 / 3").unwrap();
    assert_eq!(line.start, Some(GridPlacement::Span(2)));
    assert_eq!(line.end, Some(GridPlacement::Line(3)));
  }
}
