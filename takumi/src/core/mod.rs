//! Core types and traits for the takumi rendering system.
//!
//! This module contains fundamental types and abstractions that are used
//! throughout the rendering pipeline, including context management and
//! viewport definitions.

/// Context management and global configuration
pub mod context;
/// Font management and rendering
pub mod fonts;
/// Image storage and caching
pub mod stores;
/// Viewport definitions and rendering context
pub mod viewport;

pub use context::*;
pub use fonts::*;
pub use stores::*;
pub use viewport::*;
