//! Style system for the takumi rendering library.
//!
//! This module contains all styling-related functionality including:
//! - Style properties and values
//! - Color management and gradients
//! - Length units and measurements
//! - CSS-like styling abstractions

/// Macros for style-related code generation
pub mod macros;
/// Style properties and CSS-like attributes
pub mod properties;

pub use properties::*;
