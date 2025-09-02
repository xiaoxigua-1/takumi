use std::fmt::Formatter;

use parley::style::FontWeight as ParleyFontWeight;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ts_rs::TS;

/// Represents font weight value.
#[derive(Debug, Default, Copy, Clone, TS, PartialEq)]
#[ts(type = "string | number")]
pub struct FontWeight(ParleyFontWeight);

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

impl<'de> Deserialize<'de> for FontWeight {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
      type Value = FontWeight;

      fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(r#""bold", "normal" or number from 0 to 1000"#)
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(FontWeight(ParleyFontWeight::parse(v).ok_or_else(|| {
          serde::de::Error::custom(format!("Invalid font weight: {v}"))
        })?))
      }

      fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(FontWeight(ParleyFontWeight::new(v as f32)))
      }
    }

    deserializer.deserialize_any(Visitor)
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
