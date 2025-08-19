use cssparser::Parser;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::layout::style::{FromCss, ParseResult};

use super::GridPlacement;

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

  #[test]
  fn test_parse_line() {
    let mut input = ParserInput::new("span 2 / 3");
    let mut parser = Parser::new(&mut input);
    let line = GridLine::from_css(&mut parser).unwrap();
    assert_eq!(line.start, Some(GridPlacement::Span(2)));
    assert_eq!(line.end, Some(GridPlacement::Line(3)));
  }
}
