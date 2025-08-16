use cssparser::{Parser, Token};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{FromCss, properties::ParseResult};

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

#[cfg(test)]
mod tests {
  use cssparser::ParserInput;

  use super::*;

  #[test]
  fn test_parse_placement() {
    let mut input = ParserInput::new("auto");
    let mut parser = Parser::new(&mut input);
    assert_eq!(
      GridPlacement::from_css(&mut parser).unwrap(),
      GridPlacement::Auto
    );

    let mut input = ParserInput::new("span 2");
    let mut parser = Parser::new(&mut input);
    assert_eq!(
      GridPlacement::from_css(&mut parser).unwrap(),
      GridPlacement::Span(2)
    );

    let mut input = ParserInput::new("span name");
    let mut parser = Parser::new(&mut input);
    assert_eq!(
      GridPlacement::from_css(&mut parser).unwrap(),
      GridPlacement::Span(1)
    );

    let mut input = ParserInput::new("3");
    let mut parser = Parser::new(&mut input);
    assert_eq!(
      GridPlacement::from_css(&mut parser).unwrap(),
      GridPlacement::Line(3)
    );

    let mut input = ParserInput::new("-1");
    let mut parser = Parser::new(&mut input);
    assert_eq!(
      GridPlacement::from_css(&mut parser).unwrap(),
      GridPlacement::Line(-1)
    );

    let mut input = ParserInput::new("header");
    let mut parser = Parser::new(&mut input);
    assert_eq!(
      GridPlacement::from_css(&mut parser).unwrap(),
      GridPlacement::Named("header".to_string())
    );
  }
}
