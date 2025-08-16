use cssparser::{Parser, Token};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{FromCss, properties::ParseResult};

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

#[cfg(test)]
mod tests {
  use super::*;
  use cssparser::{Parser, ParserInput};

  #[test]
  fn test_parse_repetition_count() {
    let mut input = ParserInput::new("auto-fill");
    let mut parser = Parser::new(&mut input);
    assert_eq!(
      GridRepetitionCount::from_css(&mut parser).unwrap(),
      GridRepetitionCount::AutoFill
    );

    let mut input = ParserInput::new("auto-fit");
    let mut parser = Parser::new(&mut input);
    assert_eq!(
      GridRepetitionCount::from_css(&mut parser).unwrap(),
      GridRepetitionCount::AutoFit
    );

    let mut input = ParserInput::new("3");
    let mut parser = Parser::new(&mut input);
    assert_eq!(
      GridRepetitionCount::from_css(&mut parser).unwrap(),
      GridRepetitionCount::Count(3)
    );
  }
}
