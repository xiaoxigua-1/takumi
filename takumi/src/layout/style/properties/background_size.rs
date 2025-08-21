use cssparser::{Parser, Token, match_ignore_ascii_case};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::layout::style::{FromCss, LengthUnit, ParseResult};

/// Parsed `background-size` for one layer.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, TS, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum BackgroundSize {
  /// Use the image's intrinsic size.
  #[default]
  Auto,
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

impl<'i> FromCss<'i> for BackgroundSize {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    // cover|contain|auto|<length|percentage> [<length|percentage>|auto]
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
      if let Some(v) = match_ignore_ascii_case! {&ident,
        "auto" => Some(BackgroundSize::Auto),
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
    }

    // First value length/percentage or auto via number/dimension/percentage/ident auto
    let parse_length_or_auto = |input: &mut Parser<'i, '_>| -> ParseResult<'i, LengthOrAuto> {
      let location = input.current_source_location();
      let token = input.next()?;
      match *token {
        Token::Percentage { unit_value, .. } => Ok(LengthOrAuto::Length(LengthUnit::Percentage(
          unit_value * 100.0,
        ))),
        Token::Number { value, .. } => Ok(LengthOrAuto::Length(LengthUnit::Px(value))),
        Token::Dimension {
          value, ref unit, ..
        } => {
          let unit_str: &str = unit;
          let len = match_ignore_ascii_case! {unit_str,
            "px" => LengthUnit::Px(value),
            "em" => LengthUnit::Em(value),
            "rem" => LengthUnit::Rem(value),
            "vw" => LengthUnit::Vw(value),
            "vh" => LengthUnit::Vh(value),
            _ => return Err(location.new_basic_unexpected_token_error(Token::Dimension { value, unit: unit.clone(), has_sign: false, int_value: None }).into()),
          };
          Ok(LengthOrAuto::Length(len))
        }
        Token::Ident(ref ident) if ident.eq_ignore_ascii_case("auto") => Ok(LengthOrAuto::Auto),
        _ => Err(
          location
            .new_basic_unexpected_token_error(token.clone())
            .into(),
        ),
      }
    };

    let first = parse_length_or_auto(input)?;
    let second = input
      .try_parse(parse_length_or_auto)
      .ok()
      .unwrap_or(LengthOrAuto::Auto);

    let (width, height) = match (first, second) {
      (LengthOrAuto::Length(w), LengthOrAuto::Length(h)) => (w, h),
      (LengthOrAuto::Length(w), LengthOrAuto::Auto) => (w, LengthUnit::Auto),
      (LengthOrAuto::Auto, LengthOrAuto::Length(h)) => (LengthUnit::Auto, h),
      (LengthOrAuto::Auto, LengthOrAuto::Auto) => return Ok(BackgroundSize::Auto),
    };

    Ok(BackgroundSize::Explicit { width, height })
  }
}

#[derive(Debug, Clone, Copy)]
enum LengthOrAuto {
  Length(LengthUnit),
  Auto,
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
        let mut input = cssparser::ParserInput::new(&css);
        let mut parser = cssparser::Parser::new(&mut input);
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
