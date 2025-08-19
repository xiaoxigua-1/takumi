//! Layout system for the takumi rendering library.
//!
//! This module contains the node-based layout system including:
//! - Node trait and implementations
//! - Layout measurement functionality
//! - Node type macros
//! - Container, text, and image node types

mod macros;
mod viewport;

/// Node-based layout system with flexbox support
pub mod node;

/// CSS-like styling system with colors, units, and properties
pub mod style;

pub use viewport::*;
