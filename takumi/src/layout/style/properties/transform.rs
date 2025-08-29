use cssparser::{Parser, ParserInput, Token, match_ignore_ascii_case};
use serde::{Deserialize, Serialize};
use taffy::{Layout, Rect};
use ts_rs::TS;

use crate::{
  layout::style::{FromCss, LengthUnit, ParseResult, parse_length_percentage},
  rendering::{DEFAULT_SCALE, RenderContext},
};

/// Represents a single CSS transform operation
#[derive(Debug, Clone, Deserialize, Serialize, Copy, TS)]
#[serde(rename_all = "camelCase")]
pub enum Transform {
  /// Translates an element along the X-axis and Y-axis by the specified lengths
  Translate(LengthUnit, LengthUnit),
  /// Scales an element by the specified factors
  Scale(f32, f32),
}

fn scale_rect(rect: &mut Rect<f32>, x_scale: f32, y_scale: f32) {
  rect.left *= x_scale;
  rect.right *= x_scale;
  rect.top *= y_scale;
  rect.bottom *= y_scale;
}

/// A collection of transform operations that can be applied together
#[derive(Debug, Clone, Deserialize, Serialize, TS, Default)]
#[ts(as = "TransformsValue")]
#[serde(try_from = "TransformsValue")]
pub struct Transforms(pub Vec<Transform>);

impl Transforms {
  /// Chains two transform collections together
  pub fn chain(&mut self, other: &Transforms) {
    self.0.extend_from_slice(&other.0);
  }

  /// Applies the transforms to the layout
  pub fn apply(&self, context: &mut RenderContext, layout: &mut Layout) {
    for transform in &self.0 {
      match *transform {
        Transform::Translate(x_length, y_length) => {
          layout.location.x += x_length.resolve_to_px(context, layout.size.width);
          layout.location.y += y_length.resolve_to_px(context, layout.size.height);
        }
        Transform::Scale(x_scale, y_scale) => {
          let original_size = layout.size;

          layout.size.width *= x_scale;
          layout.size.height *= y_scale;

          // assume center of the element is the origin of the transform
          layout.location.x -= (layout.size.width - original_size.width) / 2.0;
          layout.location.y -= (layout.size.height - original_size.height) / 2.0;

          scale_rect(&mut layout.border, x_scale, y_scale);
          scale_rect(&mut layout.padding, x_scale, y_scale);
          scale_rect(&mut layout.margin, x_scale, y_scale);

          // update the scale of the render context
          context.scale.width *= x_scale;
          context.scale.height *= y_scale;
        }
      }
    }
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
        DEFAULT_SCALE.height,
      )),
      "scaley" => Ok(Transform::Scale(
        DEFAULT_SCALE.width,
        parser.parse_nested_block(parse_length_percentage)?,
      )),
      _ => Err(location.new_basic_unexpected_token_error(token.clone()).into()),
    }
  }
}
