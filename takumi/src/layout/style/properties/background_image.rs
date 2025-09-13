use cssparser::{Parser, ParserInput};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use ts_rs::TS;

use crate::layout::style::{FromCss, LinearGradient, NoiseV1, ParseResult, RadialGradient};

/// Background image variants supported by Takumi.
#[derive(Debug, Clone, PartialEq, TS, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BackgroundImage {
  /// CSS linear-gradient(...)
  Linear(LinearGradient),
  /// CSS radial-gradient(...)
  Radial(RadialGradient),
  /// Custom noise-v1(...)
  Noise(NoiseV1),
}

impl<'i> FromCss<'i> for BackgroundImage {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, BackgroundImage> {
    if let Ok(gradient) = input.try_parse(LinearGradient::from_css) {
      return Ok(BackgroundImage::Linear(gradient));
    }
    if let Ok(gradient) = input.try_parse(RadialGradient::from_css) {
      return Ok(BackgroundImage::Radial(gradient));
    }
    if let Ok(noise) = input.try_parse(NoiseV1::from_css) {
      return Ok(BackgroundImage::Noise(noise));
    }
    // TODO: url(...) images can be supported here later
    Err(input.new_error(cssparser::BasicParseErrorKind::QualifiedRuleInvalid))
  }
}

/// Proxy type to deserialize CSS background images as either a list or CSS string
#[derive(Debug, Clone, PartialEq, TS, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum BackgroundImagesValue {
  /// Structured variant: explicit list of background images
  #[ts(as = "Vec<BackgroundImage>")]
  Images(SmallVec<[BackgroundImage; 4]>),
  /// CSS string variant
  Css(String),
}

/// A collection of background images.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, TS)]
#[ts(as = "BackgroundImagesValue")]
#[serde(try_from = "BackgroundImagesValue")]
pub struct BackgroundImages(pub SmallVec<[BackgroundImage; 4]>);

impl TryFrom<BackgroundImagesValue> for BackgroundImages {
  type Error = String;

  fn try_from(value: BackgroundImagesValue) -> Result<Self, Self::Error> {
    match value {
      BackgroundImagesValue::Images(images) => Ok(Self(images)),
      BackgroundImagesValue::Css(css) => {
        let mut input = ParserInput::new(&css);
        let mut parser = Parser::new(&mut input);

        let mut images = SmallVec::new();
        images.push(BackgroundImage::from_css(&mut parser).map_err(|e| e.to_string())?);

        while parser.expect_comma().is_ok() {
          images.push(BackgroundImage::from_css(&mut parser).map_err(|e| e.to_string())?);
        }

        Ok(Self(images))
      }
    }
  }
}
