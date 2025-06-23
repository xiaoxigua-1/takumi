//! Visual effects system for the takumi rendering library.
//!
//! This module contains all visual effects functionality including:
//! - Border radius calculations and rendering
//! - Box shadow effects and blur operations
//! - Border drawing and styling
//! - Anti-aliased rendering effects

/// Border drawing and styling
pub mod border;
/// Border radius calculations and rendering
pub mod radius;
/// Box shadow effects and blur operations
pub mod shadow;

pub use border::*;
pub use radius::*;
pub use shadow::*;
