use cosmic_text::Weight;
use cssparser::{Parser, Token, match_ignore_ascii_case};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{FromCss, properties::ParseResult};

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

impl<'i> FromCss<'i> for FontWeight {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    let location = input.current_source_location();
    let token = input.next()?;

    match *token {
      Token::Ident(ref ident) => match_ignore_ascii_case! {&ident,
        "normal" => Ok(FontWeight(400)),
        "bold" => Ok(FontWeight(700)),
        "bolder" => Ok(FontWeight(700)),
        "lighter" => Ok(FontWeight(300)),
        _ => Err(location.new_basic_unexpected_token_error(token.clone()).into()),
      },
      Token::Number { value, .. } => {
        // Only accept 100..=900 in 100 increments
        let int_value = value as i32;
        if (100..=900).contains(&int_value) && int_value % 100 == 0 {
          Ok(FontWeight(int_value as u16))
        } else {
          Err(
            location
              .new_basic_unexpected_token_error(token.clone())
              .into(),
          )
        }
      }
      _ => Err(
        location
          .new_basic_unexpected_token_error(token.clone())
          .into(),
      ),
    }
  }
}
