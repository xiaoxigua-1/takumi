use cssparser::{Parser, Token, match_ignore_ascii_case};

use crate::{LengthUnit, properties::ParseResult};

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

/// Parses a length unit (e.g., "px", "em", "rem") from the input.
pub fn parse_length_unit<'i>(input: &mut Parser<'i, '_>) -> ParseResult<'i, LengthUnit> {
  let location = input.current_source_location();
  let token = input.next()?;

  match *token {
    Token::Ident(ref unit) => match_ignore_ascii_case! {&unit,
      "auto" => Ok(LengthUnit::Auto),
      "min-content" => Ok(LengthUnit::MinContent),
      "max-content" => Ok(LengthUnit::MaxContent),
      _ => Err(location.new_basic_unexpected_token_error(token.clone()).into()),
    },
    Token::Dimension {
      value, ref unit, ..
    } => {
      match_ignore_ascii_case! {&unit,
        "px" => Ok(LengthUnit::Px(value)),
        "em" => Ok(LengthUnit::Em(value)),
        "rem" => Ok(LengthUnit::Rem(value)),
        "vw" => Ok(LengthUnit::Vw(value)),
        "vh" => Ok(LengthUnit::Vh(value)),
        _ => Err(location.new_basic_unexpected_token_error(token.clone()).into()),
      }
    }
    Token::Percentage { unit_value, .. } => Ok(LengthUnit::Percentage(unit_value * 100.0)),
    Token::Number { value, .. } => Ok(LengthUnit::Px(value)),
    _ => Err(
      location
        .new_basic_unexpected_token_error(token.clone())
        .into(),
    ),
  }
}
