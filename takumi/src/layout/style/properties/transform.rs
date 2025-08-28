use cssparser::{Parser, ParserInput, Token, match_ignore_ascii_case};
use serde::{Deserialize, Serialize};
use taffy::Point;
use ts_rs::TS;

use crate::{
  layout::style::{FromCss, LengthUnit, ParseResult},
  rendering::RenderContext,
};

/// Represents a single CSS transform operation
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub enum Transform {
  /// Translates an element along the X-axis and Y-axis by the specified lengths
  Translate(LengthUnit, LengthUnit),
}

/// A collection of transform operations that can be applied together
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct Transforms(pub Vec<Transform>);

impl Transforms {
  /// Calculates the total translation point by combining all transform operations
  pub fn translate(&self, context: &RenderContext) -> Point<f32> {
    let mut x = 0.0;
    let mut y = 0.0;

    for transform in &self.0 {
      match *transform {
        Transform::Translate(x_length, y_length) => {
          x += x_length.resolve_to_px(context);
          y += y_length.resolve_to_px(context);
        }
      }
    }

    Point { x, y }
  }
}

/// Represents transform values that can be either a structured list or raw CSS
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(untagged)]
pub enum TransformsValue {
  /// A structured list of transform operations
  Transforms(Vec<Transform>),
  /// Raw CSS transform string to be parsed
  Css(String),
}

impl TryFrom<TransformsValue> for Transforms {
  type Error = String;

  fn try_from(value: TransformsValue) -> Result<Self, Self::Error> {
    match value {
      TransformsValue::Transforms(transforms) => Ok(Transforms(transforms)),
      TransformsValue::Css(css) => {
        let mut input = ParserInput::new(&css);
        let mut parser = Parser::new(&mut input);

        let mut transforms = Vec::new();

        while !parser.is_exhausted() {
          let transform = Transform::from_css(&mut parser).map_err(|e| e.to_string())?;
          transforms.push(transform);
        }

        Ok(Transforms(transforms))
      }
    }
  }
}

impl<'i> FromCss<'i> for Transform {
  fn from_css(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    let location = parser.current_source_location();
    let token = parser.next()?;

    let Token::Function(function) = token else {
      return Err(
        location
          .new_basic_unexpected_token_error(token.clone())
          .into(),
      );
    };

    match_ignore_ascii_case! {function,
      "translate" => Ok(Transform::Translate(
        parser.parse_nested_block(LengthUnit::from_css)?,
        parser.parse_nested_block(LengthUnit::from_css)?,
      )),
      "translatex" => Ok(Transform::Translate(
        parser.parse_nested_block(LengthUnit::from_css)?,
        LengthUnit::zero(),
      )),
      "translatey" => Ok(Transform::Translate(
        LengthUnit::zero(),
        parser.parse_nested_block(LengthUnit::from_css)?,
      )),
      _ => Err(location.new_basic_unexpected_token_error(token.clone()).into()),
    }
  }
}
