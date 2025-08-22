use cssparser::{Parser, ParserInput, Token, match_ignore_ascii_case};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::layout::style::{FromCss, LengthUnit, ParseResult};

/// Parsed `background-size` for one layer.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, TS, PartialEq)]
pub enum BackgroundSize {
  /// Scale the image to cover the container (may crop).
  Cover,
  /// Scale the image to be fully contained within the container.
  Contain,
  /// Explicit width and height values.
  Explicit {
    /// Width value for the background image.
    width: LengthUnit,
    /// Height value for the background image.
    height: LengthUnit,
  },
}

impl Default for BackgroundSize {
  fn default() -> Self {
    BackgroundSize::Explicit {
      width: LengthUnit::Auto,
      height: LengthUnit::Auto,
    }
  }
}

impl<'i> FromCss<'i> for BackgroundSize {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    // skip "auto" as it could have a second value, and it should be treated as a LengthUnit
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned)
      && !ident.eq_ignore_ascii_case("auto")
    {
      if let Some(v) = match_ignore_ascii_case! {&ident,
        "cover" => Some(BackgroundSize::Cover),
        "contain" => Some(BackgroundSize::Contain),
        _ => None,
      } {
        return Ok(v);
      }
      // fallback to treating ident as error
      let location = input.current_source_location();
      return Err(
        location
          .new_basic_unexpected_token_error(Token::Ident(ident))
          .into(),
      );
    };

    let first = LengthUnit::from_css(input)?;
    let Ok(second) = input.try_parse(LengthUnit::from_css) else {
      return Ok(BackgroundSize::Explicit {
        width: first,
        height: first,
      });
    };

    Ok(BackgroundSize::Explicit {
      width: first,
      height: second,
    })
  }
}

/// A value representing either a list of parsed sizes or a raw CSS string.
#[derive(Debug, Clone, PartialEq, TS, Deserialize)]
#[serde(untagged)]
pub enum BackgroundSizesValue {
  /// Parsed sizes for one or more layers.
  Sizes(Vec<BackgroundSize>),
  /// Raw CSS to be parsed at runtime.
  Css(String),
}

/// A list of `background-size` values (one per layer).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, TS)]
#[ts(as = "BackgroundSizesValue")]
#[serde(try_from = "BackgroundSizesValue")]
pub struct BackgroundSizes(pub Vec<BackgroundSize>);

impl TryFrom<BackgroundSizesValue> for BackgroundSizes {
  type Error = &'static str;

  fn try_from(value: BackgroundSizesValue) -> Result<Self, Self::Error> {
    match value {
      BackgroundSizesValue::Sizes(v) => Ok(Self(v)),
      BackgroundSizesValue::Css(css) => {
        let mut input = ParserInput::new(&css);
        let mut parser = Parser::new(&mut input);
        let mut values = vec![
          BackgroundSize::from_css(&mut parser)
            .map_err(|_| "Failed to parse first background-size")?,
        ];
        while parser.expect_comma().is_ok() {
          values.push(
            BackgroundSize::from_css(&mut parser).map_err(|_| "Failed to parse background-size")?,
          );
        }
        Ok(Self(values))
      }
    }
  }
}
