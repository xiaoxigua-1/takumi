mod properties;
mod stylesheets;

use cssparser::Parser;
pub use properties::*;
use serde::{Deserialize, Serialize};
pub use stylesheets::*;
use ts_rs::TS;

/// Represents a CSS property value that can be explicitly set, inherited from parent, or reset to initial value.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum CssValue<T> {
  /// Inherit the computed value from the parent element
  Inherit,
  /// Reset to the property's initial/default value
  Unset,
  /// Explicit value set on the element
  #[serde(untagged)]
  Value(T),
}

impl<'i, T: FromCss<'i>> FromCss<'i> for CssValue<T> {
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
    if input
      .try_parse(|input| input.expect_ident_matching("inherit"))
      .is_ok()
    {
      return Ok(CssValue::Inherit);
    }

    if input
      .try_parse(|input| input.expect_ident_matching("unset"))
      .is_ok()
    {
      return Ok(CssValue::Unset);
    }

    T::from_css(input).map(CssValue::Value)
  }
}

impl<T> From<T> for CssValue<T> {
  fn from(value: T) -> Self {
    CssValue::Value(value)
  }
}

impl<T> CssValue<T> {
  /// Resolves this CssValue to a concrete value based on inheritance rules
  pub fn inherit(&self, parent: &T, initial_value: T) -> T
  where
    T: Clone,
  {
    match self {
      Self::Value(v) => v.clone(),
      Self::Inherit => parent.clone(),
      Self::Unset => initial_value,
    }
  }
}

impl<T: Default> Default for CssValue<T> {
  fn default() -> Self {
    Self::Unset
  }
}

impl<T: Copy> Copy for CssValue<T> {}
