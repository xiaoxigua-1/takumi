use cssparser::{Parser, Token};

use crate::properties::ParseResult;

/// Parses a length (0.0-1.0) or percentage value (0%-100%) from the input.
pub fn parse_length_percentage<'i>(input: &mut Parser<'i, '_>) -> ParseResult<'i, f32> {
  let location = input.current_source_location();
  let token = input.next()?;

  match token {
    Token::Number { value, .. } => Ok(value.max(0.0)),
    Token::Percentage { unit_value, .. } => Ok(unit_value.max(0.0)),
    _ => Err(
      location
        .new_basic_unexpected_token_error(token.clone())
        .into(),
    ),
  }
}
