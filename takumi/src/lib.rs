//! # takumi
//!
//! High-performance Rust library for generating images with CSS Flexbox-like layouts.
//!
//! _Takumi (匠) means "artisan" or "craftsman" in Japanese - reflecting the precision and artistry required to craft beautiful images through code._
//!
//! Checkout the [minimal example](https://github.com/kane50613/takumi/blob/master/example/src/minimal.rs) for a quick start.
//!
//! ## Module Organization
//!
//! The library is organized into the following modules:
//!
//! - [`core`] - Core types, traits, and configuration
//! - [`style`] - CSS-like styling system with colors, units, and properties
//! - [`layout`] - Node-based layout system with flexbox support
//! - [`rendering`] - High-performance image rendering and canvas operations
//! - [`effects`] - Visual effects like borders, shadows, and radius
//! - [`resources`] - External resource management (fonts, images)
//!
//! ## Credits
//!
//! - [taffy](https://github.com/DioxusLabs/taffy) for the layout system.
//! - [image](https://github.com/image-rs/image) for the image processing.
//! - [cosmic-text](https://github.com/kornelski/cosmic-text) for the text rendering.
//! - [woff2-patched](https://github.com/zimond/woff2-rs) for the font processing.
//! - [ts-rs](https://github.com/AlephAlpha/ts-rs) for the type-safe serialization.

#![deny(missing_docs)]

/// Core types and traits for the takumi rendering system
pub mod core;

/// CSS-like styling system with colors, units, and properties
pub mod style;

/// Node-based layout system with flexbox support
pub mod layout;

/// High-performance image rendering and canvas operations
pub mod rendering;

/// Visual effects like borders, shadows, and radius
pub mod effects;

/// External resource management (fonts, images)
pub mod resources;

// Re-export commonly used types from each module
pub use core::{GlobalContext, ImageStore, RenderContext, Viewport};
pub use effects::BorderRadius;
pub use layout::{
  ContainerNode, DefaultNodeKind, ImageNode, Node, TextNode, measure_image, measure_text,
};
pub use rendering::{FastBlendImage, ImageRenderer};
pub use resources::{FontError, ImageState};
pub use style::{
  AlignItems, BoxShadow, BoxShadowInput, Color, ColorInput, FlexDirection, FlexWrap, FontWeight,
  Gap, Gradient, InheritableStyle, JustifyContent, LengthUnit, ObjectFit, Position,
  ResolvedFontStyle, SidesValue, Style, TextAlign,
};

// Re-export external dependencies for convenience
pub use image;
pub use taffy;
