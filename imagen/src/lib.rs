//! takumi is a Rust library for rendering images with a layout system similar to CSS.
//! It provides a flexible way to create and render images with various styling options,
//! including colors, fonts, borders, and layout management.
//!
//! The library uses Taffy for layout calculations and supports features like:
//! - Custom fonts with WOFF2 support
//! - Color management with RGB/RGBA support
//! - Border radius styling
//! - Image rendering and caching
//! - Debug visualization tools

#![deny(missing_docs)]

/// Module for handling border radius calculations and styling
pub(crate) mod border_radius;

/// Module for color management and conversion between different color formats
pub mod color;

/// Module for managing rendering context, including font systems and image storage
pub mod context;

/// Module for font loading and management, with support for WOFF2 fonts
pub mod font;

/// Module for node-based layout system and drawing primitives
pub mod node;

/// Module for image rendering and canvas management
pub mod render;

pub use image;
pub use taffy;
