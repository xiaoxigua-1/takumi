//! # takumi
//!
//! High-performance Rust library for generating images with CSS Flexbox-like layouts.
//!
//! _Takumi (匠) means "artisan" or "craftsman" in Japanese - reflecting the precision and artistry required to craft beautiful images through code._
//!
//! Checkout the [minimal example](https://github.com/kane50613/takumi/blob/master/example/src/minimal.rs) for a quick start.
//!
//! ## Credits
//!
//! - [taffy](https://github.com/DioxusLabs/taffy) for the layout system.
//! - [image](https://github.com/image-rs/image) for the image processing.
//! - [cosmic-text](https://github.com/kornelski/cosmic-text) for the text rendering.
//! - [woff2-patched](https://github.com/zimond/woff2-rs) for the font processing.
//! - [ts-rs](https://github.com/AlephAlpha/ts-rs) for the type-safe serialization.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::use_self)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]

/// Node-based layout system with flexbox support
pub mod layout;

/// High-performance image rendering and canvas operations
pub mod rendering;

/// External resource management (fonts, images)
pub mod resources;

// Re-export external dependencies for convenience
pub use image;
pub use taffy;

use crate::{
  rendering::RenderError,
  resources::{
    font::FontContext,
    image::{ImageResourceError, PersistentImageStore},
  },
};

/// The main context for image rendering.
///
/// This struct holds all the necessary state for rendering images, including
/// font management, image storage, and debug options.
#[derive(Default)]
pub struct GlobalContext {
  /// Whether to draw debug borders around nodes
  pub draw_debug_border: bool,
  /// The font context for text rendering
  pub font_context: FontContext,
  /// The image store for persisting contents
  pub persistent_image_store: PersistentImageStore,
}

/// Represents errors that can occur.
#[derive(Debug)]
pub enum Error {
  /// Represents an error that occurs during image resolution.
  ImageResolveError(ImageResourceError),
  /// Represents an error that occurs during rendering.
  RenderError(RenderError),
}
