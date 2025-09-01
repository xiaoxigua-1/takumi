use parley::style::FontWeight as ParleyFontWeight;
use serde::{Deserialize, Serialize, Serializer};
use ts_rs::TS;

/// Represents font weight value.
#[derive(Debug, Default, Copy, Clone, Deserialize, TS, PartialEq)]
#[serde(try_from = "FontWeightValue")]
#[ts(as = "FontWeightValue")]
pub struct FontWeight(pub ParleyFontWeight);

#[derive(Debug, Copy, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum FontWeightValue {
  Thin,
  ExtraLight,
  Light,
  SemiLight,
  Normal,
  Medium,
  SemiBold,
  Bold,
  ExtraBold,
  Black,
  ExtraBlack,
  #[serde(untagged)]
  Number(f32),
}

impl From<FontWeight> for ParleyFontWeight {
  fn from(value: FontWeight) -> Self {
    value.0
  }
}

impl From<f32> for FontWeight {
  fn from(value: f32) -> Self {
    FontWeight(ParleyFontWeight::new(value))
  }
}

impl TryFrom<FontWeightValue> for FontWeight {
  type Error = String;

  fn try_from(value: FontWeightValue) -> Result<Self, Self::Error> {
    match value {
      FontWeightValue::Number(value) => Ok(FontWeight(ParleyFontWeight::new(value))),
      FontWeightValue::Thin => Ok(FontWeight(ParleyFontWeight::THIN)),
      FontWeightValue::ExtraLight => Ok(FontWeight(parley::style::FontWeight::EXTRA_LIGHT)),
      FontWeightValue::Light => Ok(FontWeight(ParleyFontWeight::LIGHT)),
      FontWeightValue::SemiLight => Ok(FontWeight(ParleyFontWeight::SEMI_LIGHT)),
      FontWeightValue::Normal => Ok(FontWeight(ParleyFontWeight::NORMAL)),
      FontWeightValue::Medium => Ok(FontWeight(ParleyFontWeight::MEDIUM)),
      FontWeightValue::SemiBold => Ok(FontWeight(ParleyFontWeight::SEMI_BOLD)),
      FontWeightValue::Bold => Ok(FontWeight(ParleyFontWeight::BOLD)),
      FontWeightValue::ExtraBold => Ok(FontWeight(ParleyFontWeight::EXTRA_BOLD)),
      FontWeightValue::Black => Ok(FontWeight(ParleyFontWeight::BLACK)),
      FontWeightValue::ExtraBlack => Ok(FontWeight(ParleyFontWeight::EXTRA_BLACK)),
    }
  }
}

impl Serialize for FontWeight {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_f32(self.0.value())
  }
}
