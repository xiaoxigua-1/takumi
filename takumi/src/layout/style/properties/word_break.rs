use serde::{Deserialize, Serialize};
use swash::text::WordBreakStrength;
use ts_rs::TS;

/// Controls how text should be broken at word boundaries.
///
/// Corresponds to CSS word-break property.
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(from = "WordBreakValue", into = "WordBreakValue")]
#[ts(as = "WordBreakValue")]
pub struct WordBreak(pub WordBreakStrength);

#[derive(Debug, Copy, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum WordBreakValue {
  Normal,
  BreakAll,
  KeepAll,
}

impl From<WordBreak> for WordBreakValue {
  fn from(value: WordBreak) -> Self {
    match value.0 {
      WordBreakStrength::Normal => WordBreakValue::Normal,
      WordBreakStrength::BreakAll => WordBreakValue::BreakAll,
      WordBreakStrength::KeepAll => WordBreakValue::KeepAll,
    }
  }
}

impl From<WordBreakValue> for WordBreak {
  fn from(value: WordBreakValue) -> Self {
    match value {
      WordBreakValue::Normal => WordBreak(WordBreakStrength::Normal),
      WordBreakValue::BreakAll => WordBreak(WordBreakStrength::BreakAll),
      WordBreakValue::KeepAll => WordBreak(WordBreakStrength::KeepAll),
    }
  }
}

impl From<WordBreak> for WordBreakStrength {
  fn from(value: WordBreak) -> Self {
    value.0
  }
}
