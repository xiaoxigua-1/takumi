//! External resources management for the takumi rendering library.
//!
//! This module contains functionality for loading and managing external resources:
//! - Font loading and processing (WOFF, WOFF2, TTF, OTF)
//! - Image storage and caching systems
//! - Resource hydration and async loading
//! - Memory-efficient resource management

/// Font loading and processing functionality
pub mod font;
/// Image state and resource management
pub mod image;

pub use font::*;
pub use image::*;

#[cfg(feature = "woff")]
pub(crate) mod woff;
