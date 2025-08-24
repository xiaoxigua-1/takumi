use cssparser::Parser;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::layout::style::{FromCss, GridTrackSize, ParseResult};

/// Represents a grid repeat track
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
pub struct GridRepeatTrack {
  /// The size of the grid track
  pub size: GridTrackSize,
  /// The names for the line preceding this track within the repeat() clause
  pub names: Vec<String>,
  /// The names for the final line after the last track within the repeat() clause
  /// Only set on the last track of the repeat fragment. For other tracks this is None.
  pub end_names: Option<Vec<String>>,
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

#[cfg(test)]
mod tests {
  use crate::layout::style::GridLengthUnit;

  use super::*;
  use cssparser::{Parser, ParserInput};

  #[test]
  fn test_parse_repeat_track_with_names() {
    let mut parser_input = ParserInput::new("[a b] 1fr [c]");
    let mut parser = Parser::new(&mut parser_input);
    let track = GridRepeatTrack::from_css(&mut parser).unwrap();
    assert_eq!(
      track.size,
      super::GridTrackSize::Fixed(GridLengthUnit::Fr(1.0))
    );
    assert_eq!(
      track.names,
      vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
  }
}
