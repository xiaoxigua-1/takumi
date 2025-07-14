//! Style system for the takumi rendering library.
//!
//! This module contains all styling-related functionality including:
//! - Style properties and values
//! - Color management and gradients
//! - Length units and measurements
//! - CSS-like styling abstractions

/// Color management and gradient definitions
pub mod color;
/// Macros for style-related code generation
pub mod macros;
/// Style properties and CSS-like attributes
pub mod properties;
/// Length units and measurement types
pub mod units;

pub use color::*;
pub use properties::*;
pub use units::*;
