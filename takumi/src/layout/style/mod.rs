//! Style system for the takumi rendering library.
//!
//! This module contains all styling-related functionality including:
//! - Style properties and values
//! - Color management and gradients
//! - Length units and measurements
//! - CSS-like styling abstractions

mod properties;
mod stylesheets;

pub use properties::*;
pub use stylesheets::*;

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
