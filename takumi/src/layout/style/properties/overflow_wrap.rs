use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Controls how text should be overflowed.
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(from = "OverflowWrapValue", into = "OverflowWrapValue")]
#[ts(as = "OverflowWrapValue")]
pub struct OverflowWrap(parley::OverflowWrap);

#[derive(Debug, Copy, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum OverflowWrapValue {
  Normal,
  Anywhere,
  BreakWord,
}

impl From<OverflowWrap> for OverflowWrapValue {
  fn from(value: OverflowWrap) -> Self {
    match value.0 {
      parley::OverflowWrap::Normal => OverflowWrapValue::Normal,
      parley::OverflowWrap::Anywhere => OverflowWrapValue::Anywhere,
      parley::OverflowWrap::BreakWord => OverflowWrapValue::BreakWord,
    }
  }
}

impl From<OverflowWrapValue> for OverflowWrap {
  fn from(value: OverflowWrapValue) -> Self {
    match value {
      OverflowWrapValue::Normal => OverflowWrap(parley::OverflowWrap::Normal),
      OverflowWrapValue::Anywhere => OverflowWrap(parley::OverflowWrap::Anywhere),
      OverflowWrapValue::BreakWord => OverflowWrap(parley::OverflowWrap::BreakWord),
    }
  }
}

impl From<OverflowWrap> for parley::OverflowWrap {
  fn from(value: OverflowWrap) -> Self {
    value.0
  }
}
