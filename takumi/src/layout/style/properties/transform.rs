use cssparser::{Parser, ParserInput, Token, match_ignore_ascii_case};
use serde::{Deserialize, Serialize};
use taffy::Layout;
use ts_rs::TS;

use crate::{
  layout::style::{
    Angle, BackgroundPosition, FromCss, LengthUnit, ParseResult, parse_length_percentage,
  },
  rendering::RenderContext,
};

/// Represents a single CSS transform operation
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS)]
#[serde(rename_all = "camelCase")]
pub enum Transform {
  /// Translates an element along the X-axis and Y-axis by the specified lengths
  Translate(LengthUnit, LengthUnit),
  /// Scales an element by the specified factors
  Scale(f32, f32),
  /// Rotates an element (2D rotation) by angle in degrees
  Rotate(Angle),
}

/// A collection of transform operations that can be applied together
#[derive(Debug, Clone, Deserialize, Serialize, TS, Default)]
#[ts(as = "TransformsValue")]
#[serde(try_from = "TransformsValue")]
pub struct Transforms(pub Vec<Transform>);

impl Transforms {
  /// Converts the transforms to a [`zeno::Transform`] instance
  pub fn to_zeno(
    &self,
    context: &RenderContext,
    layout: &Layout,
    transform_origin: BackgroundPosition,
  ) -> zeno::Transform {
    let width = layout.size.width / 2.0;
    let height = layout.size.height / 2.0;

    let transform_origin_x = transform_origin
      .x
      .to_length_unit()
      .resolve_to_px(context, layout.size.width)
      + width;
    let transform_origin_y = transform_origin
      .y
      .to_length_unit()
      .resolve_to_px(context, layout.size.height)
      + height;

    let mut instance = zeno::Transform::translation(transform_origin_x, transform_origin_y);

    for transform in self.0.iter().rev() {
      match *transform {
        Transform::Translate(x_length, y_length) => {
          instance = instance.then_translate(
            x_length.resolve_to_px(context, layout.size.width),
            y_length.resolve_to_px(context, layout.size.height),
          );
        }
        Transform::Scale(x_scale, y_scale) => {
          instance = instance.then_scale(x_scale, y_scale);
        }
        Transform::Rotate(angle) => {
          instance = instance.then_rotate(zeno::Angle::from_degrees(*angle));
        }
      }
    }

    instance.then_translate(-transform_origin_x, -transform_origin_y)
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
      "scale" => Ok(Transform::Scale(
        parser.parse_nested_block(parse_length_percentage)?,
        parser.parse_nested_block(parse_length_percentage)?,
      )),
      "scalex" => Ok(Transform::Scale(
        parser.parse_nested_block(parse_length_percentage)?,
        1.0,
      )),
      "scaley" => Ok(Transform::Scale(
        1.0,
        parser.parse_nested_block(parse_length_percentage)?,
      )),
      "rotate" => Ok(Transform::Rotate(
        parser.parse_nested_block(Angle::from_css)?,
      )),
      _ => Err(location.new_basic_unexpected_token_error(token.clone()).into()),
    }
  }
}
