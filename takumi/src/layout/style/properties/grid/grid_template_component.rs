use cssparser::{Parser, ParserInput};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::layout::style::{FromCss, ParseResult};

use super::{GridRepeatTrack, GridRepetitionCount, GridTrackSize};

/// A transparent wrapper around a list of `GridTemplateComponent`.
///
/// This exists to provide a distinct type for template component lists while
/// preserving JSON compatibility (serialized as a plain array) and clean TS types.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(try_from = "GridTemplateComponentsValue")]
#[ts(as = "GridTemplateComponentsValue")]
pub struct GridTemplateComponents(pub Vec<GridTemplateComponent>);

/// Serializable input for `GridTemplateComponents` that accepts either a
/// pre-parsed component list or a CSS string to be parsed at runtime.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(untagged)]
pub enum GridTemplateComponentsValue {
  /// Explicit list of template components.
  Components(Vec<GridTemplateComponent>),
  /// CSS value to parse (e.g. "[a] 1fr [b] 2fr" or "repeat(3, 1fr)").
  Css(String),
}

impl TryFrom<GridTemplateComponentsValue> for GridTemplateComponents {
  type Error = &'static str;

  fn try_from(value: GridTemplateComponentsValue) -> Result<Self, Self::Error> {
    match value {
      GridTemplateComponentsValue::Components(components) => Ok(GridTemplateComponents(components)),
      GridTemplateComponentsValue::Css(css) => {
        let mut input = ParserInput::new(&css);
        let mut parser = Parser::new(&mut input);

        let mut components = Vec::new();

        while let Ok(component) = GridTemplateComponent::from_css(&mut parser) {
          components.push(component);
        }

        Ok(GridTemplateComponents(components))
      }
    }
  }
}

/// Represents a track sizing function or a list of line names between tracks
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(untagged)]
pub enum GridTemplateComponent {
  /// A list of line names that apply to the current grid line (e.g., [a b])
  LineNames(Vec<String>),
  /// A single non-repeated track
  Single(GridTrackSize),
  /// Automatically generate grid tracks to fit the available space using the specified definite track lengths
  /// Only valid if every track in template (not just the repetition) has a fixed size.
  Repeat(GridRepetitionCount, Vec<GridRepeatTrack>),
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

#[cfg(test)]
mod tests {
  use super::*;
  use cssparser::{Parser, ParserInput};

  #[test]
  fn test_parse_template_component_repeat() {
    let mut input = ParserInput::new("repeat(auto-fill, [a] 1fr [b] 2fr)");
    let mut parser = Parser::new(&mut input);
    let tpl = GridTemplateComponent::from_css(&mut parser).unwrap();
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
}
