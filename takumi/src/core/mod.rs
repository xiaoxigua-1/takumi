//! Core types and traits for the takumi rendering system.
//!
//! This module contains fundamental types and abstractions that are used
//! throughout the rendering pipeline, including context management and
//! viewport definitions.

/// Context management and global configuration
pub mod context;
/// Font Context
pub mod font_context;
/// Image storage and caching
pub mod image_store;
/// Viewport definitions and rendering context
pub mod viewport;

pub use context::*;
pub use font_context::*;
pub use image_store::*;
pub use viewport::*;
