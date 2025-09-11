//! Style system for the takumi rendering library.
//!
//! This module contains all styling-related functionality including:
//! - Style properties and values
//! - Color management and gradients
//! - Length units and measurements
//! - CSS-like styling abstractions

mod macros;
mod properties;
mod stylesheets;

use cssparser::Parser;
pub use properties::*;
use serde::{Deserialize, Serialize};
pub use stylesheets::*;
use ts_rs::TS;

/// Represents a CSS property value that can be explicitly set, inherited from parent, or reset to initial value.
#[derive(Debug, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(untagged)]
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
  /// Returns true if this is an explicit value
  pub fn is_value(&self) -> bool {
    matches!(self, Self::Value(_))
  }

  /// Returns true if this inherits from parent
  pub fn is_inherit(&self) -> bool {
    matches!(self, Self::Inherit)
  }

  /// Returns true if this resets to initial value
  pub fn is_unset(&self) -> bool {
    matches!(self, Self::Unset)
  }

  /// Gets the inner value if this is a Value variant, None otherwise
  pub fn as_value(&self) -> Option<&T> {
    match self {
      Self::Value(v) => Some(v),
      _ => None,
    }
  }

  /// Maps the inner value using the provided function
  pub fn map<U, F>(self, f: F) -> CssValue<U>
  where
    F: FnOnce(T) -> U,
  {
    match self {
      Self::Value(v) => CssValue::Value(f(v)),
      Self::Inherit => CssValue::Inherit,
      Self::Unset => CssValue::Unset,
    }
  }

  /// Resolves this CssValue to a concrete value based on inheritance rules
  pub fn resolve<F>(&self, inherited_value: Option<&T>, initial_value: T, parent_resolver: F) -> T
  where
    F: FnOnce() -> T,
    T: Clone,
  {
    match self {
      Self::Value(v) => v.clone(),
      Self::Inherit => inherited_value.cloned().unwrap_or_else(parent_resolver),
      Self::Unset => initial_value,
    }
  }

  /// Returns the contained value or computes it from a closure if this is Inherit or Unset
  pub fn unwrap_or_else<F>(self, f: F) -> T
  where
    F: FnOnce() -> T,
  {
    match self {
      Self::Value(v) => v,
      Self::Inherit | Self::Unset => f(),
    }
  }

  /// Returns the contained value or a default if this is Inherit or Unset
  pub fn unwrap_or(self, default: T) -> T {
    match self {
      Self::Value(v) => v,
      Self::Inherit | Self::Unset => default,
    }
  }

  /// Returns the contained value or computes it from a closure if this is Inherit or Unset (reference version)
  pub fn unwrap_or_else_ref<F>(&self, f: F) -> T
  where
    F: FnOnce() -> T,
    T: Clone,
  {
    match self {
      Self::Value(v) => v.clone(),
      Self::Inherit | Self::Unset => f(),
    }
  }

  /// Returns the contained value or a default if this is Inherit or Unset (reference version)
  pub fn unwrap_or_ref(&self, default: T) -> T
  where
    T: Clone,
  {
    match self {
      Self::Value(v) => v.clone(),
      Self::Inherit | Self::Unset => default,
    }
  }

  /// Returns the contained value or default if this is Inherit or Unset (reference version)
  pub fn unwrap_or_default_ref(&self) -> T
  where
    T: Default + Clone,
  {
    match self {
      Self::Value(v) => v.clone(),
      Self::Inherit | Self::Unset => T::default(),
    }
  }
}

impl<T: Default> Default for CssValue<T> {
  fn default() -> Self {
    Self::Unset
  }
}

impl<T: Copy> Copy for CssValue<T> {}

/// Macro to implement From trait for Taffy enum conversions
#[macro_export]
macro_rules! impl_from_taffy_enum {
  ($from_ty:ty, $to_ty:ty, $($variant:ident),*) => {
    impl From<$from_ty> for $to_ty {
      fn from(value: $from_ty) -> Self {
        match value {
          $(<$from_ty>::$variant => <$to_ty>::$variant,)*
        }
      }
    }
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use cssparser::{Parser, ParserInput};

  #[test]
  fn test_css_value_parsing_inherit() {
    let mut input = ParserInput::new("inherit");
    let mut parser = Parser::new(&mut input);

    let result: CssValue<LengthUnit> = CssValue::from_css(&mut parser).unwrap();
    assert!(matches!(result, CssValue::Inherit));
    assert!(result.is_inherit());
    assert!(!result.is_value());
    assert!(!result.is_unset());
  }

  #[test]
  fn test_css_value_parsing_unset() {
    let mut input = ParserInput::new("unset");
    let mut parser = Parser::new(&mut input);

    let result: CssValue<LengthUnit> = CssValue::from_css(&mut parser).unwrap();
    assert!(matches!(result, CssValue::Unset));
    assert!(result.is_unset());
    assert!(!result.is_value());
    assert!(!result.is_inherit());
  }

  #[test]
  fn test_css_value_parsing_value() {
    let mut input = ParserInput::new("42px");
    let mut parser = Parser::new(&mut input);

    let result: CssValue<LengthUnit> = CssValue::from_css(&mut parser).unwrap();
    assert!(result.is_value());
    assert!(!result.is_inherit());
    assert!(!result.is_unset());
    assert_eq!(result.as_value(), Some(&LengthUnit::Px(42.0)));
  }

  #[test]
  fn test_css_value_constructors() {
    let value = CssValue::Value(LengthUnit::Px(42.0));
    assert!(matches!(value, CssValue::Value(LengthUnit::Px(42.0))));

    let unset = CssValue::<LengthUnit>::Unset;
    assert!(matches!(unset, CssValue::Unset));
  }

  #[test]
  fn test_css_value_default() {
    let default_value: CssValue<LengthUnit> = Default::default();
    assert!(matches!(default_value, CssValue::Unset));
  }

  #[test]
  fn test_css_value_resolve() {
    let value = CssValue::Value(LengthUnit::Px(42.0));
    let resolved = value.resolve(Some(&LengthUnit::Px(24.0)), LengthUnit::Px(12.0), || {
      LengthUnit::Px(6.0)
    });
    assert_eq!(resolved, LengthUnit::Px(42.0));

    let unset = CssValue::<LengthUnit>::Unset;
    let resolved = unset.resolve(Some(&LengthUnit::Px(24.0)), LengthUnit::Px(12.0), || {
      LengthUnit::Px(6.0)
    });
    assert_eq!(resolved, LengthUnit::Px(12.0)); // Uses initial value
  }

  #[test]
  fn test_css_value_unwrap_or_else() {
    let value = CssValue::Value(LengthUnit::Px(42.0));
    let result = value.unwrap_or_else(|| LengthUnit::Px(24.0));
    assert_eq!(result, LengthUnit::Px(42.0));

    let unset = CssValue::<LengthUnit>::Unset;
    let result = unset.unwrap_or_else(|| LengthUnit::Px(24.0));
    assert_eq!(result, LengthUnit::Px(24.0));
  }

  #[test]
  fn test_css_value_unwrap_or() {
    let value = CssValue::Value(LengthUnit::Px(42.0));
    let result = value.unwrap_or(LengthUnit::Px(24.0));
    assert_eq!(result, LengthUnit::Px(42.0));

    let unset = CssValue::<LengthUnit>::Unset;
    let result = unset.unwrap_or(LengthUnit::Px(24.0));
    assert_eq!(result, LengthUnit::Px(24.0));
  }

  #[test]
  fn test_css_value_unwrap_or_else_ref() {
    let value = CssValue::Value(LengthUnit::Px(42.0));
    let result = value.unwrap_or_else_ref(|| LengthUnit::Px(24.0));
    assert_eq!(result, LengthUnit::Px(42.0));

    let unset = CssValue::<LengthUnit>::Unset;
    let result = unset.unwrap_or_else_ref(|| LengthUnit::Px(24.0));
    assert_eq!(result, LengthUnit::Px(24.0));
  }

  #[test]
  fn test_css_value_unwrap_or_ref() {
    let value = CssValue::Value(LengthUnit::Px(42.0));
    let result = value.unwrap_or_ref(LengthUnit::Px(24.0));
    assert_eq!(result, LengthUnit::Px(42.0));

    let unset = CssValue::<LengthUnit>::Unset;
    let result = unset.unwrap_or_ref(LengthUnit::Px(24.0));
    assert_eq!(result, LengthUnit::Px(24.0));
  }

  #[test]
  fn test_css_value_unwrap_or_default_ref() {
    let value = CssValue::Value(LengthUnit::Px(42.0));
    let result = value.unwrap_or_default_ref();
    assert_eq!(result, LengthUnit::Px(42.0));

    let unset = CssValue::<LengthUnit>::Unset;
    let result = unset.unwrap_or_default_ref();
    assert_eq!(result, LengthUnit::Auto); // LengthUnit default
  }

  #[test]
  fn test_css_value_map() {
    let value = CssValue::Value(LengthUnit::Px(42.0));
    let mapped = value.map(|x| match x {
      LengthUnit::Px(v) => LengthUnit::Px(v * 2.0),
      _ => x,
    });
    assert!(matches!(mapped, CssValue::Value(LengthUnit::Px(84.0))));

    let unset = CssValue::<LengthUnit>::Unset;
    let mapped = unset.map(|x| x);
    assert!(matches!(mapped, CssValue::Unset));
  }

  #[test]
  fn test_css_value_as_value() {
    let value = CssValue::Value(LengthUnit::Px(42.0));
    assert_eq!(value.as_value(), Some(&LengthUnit::Px(42.0)));

    let unset = CssValue::<LengthUnit>::Unset;
    assert_eq!(unset.as_value(), None);
  }

  #[test]
  fn test_css_value_is_methods() {
    let value = CssValue::Value(LengthUnit::Px(42.0));
    assert!(value.is_value());
    assert!(!value.is_unset());

    let unset = CssValue::<LengthUnit>::Unset;
    assert!(!unset.is_value());
    assert!(unset.is_unset());
  }

  #[test]
  fn test_css_value_serialization() {
    // Test that CssValue can be serialized/deserialized
    let value = CssValue::Value(42.0);
    let serialized = serde_json::to_string(&value).unwrap();
    assert_eq!(serialized, "42.0");

    let unset = CssValue::<f32>::Unset;
    let serialized = serde_json::to_string(&unset).unwrap();
    assert_eq!(serialized, "\"Unset\"");
  }

  #[test]
  fn test_css_value_deserialization() {
    // Test deserialization
    let value: CssValue<f32> = serde_json::from_str("42.0").unwrap();
    assert!(matches!(value, CssValue::Value(42.0)));

    let unset: CssValue<f32> = serde_json::from_str("\"Unset\"").unwrap();
    assert!(matches!(unset, CssValue::Unset));
  }
}
