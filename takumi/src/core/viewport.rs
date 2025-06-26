//! Viewport definitions and related types for the takumi rendering system.
//!
//! This module contains the viewport type and related context structures
//! that define the rendering area and font sizing.

/// The default font size in pixels.
pub const DEFAULT_FONT_SIZE: f32 = 16.0;

/// The default line height multiplier.
pub const DEFAULT_LINE_HEIGHT: f32 = 1.0;

/// The viewport for the image renderer.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct Viewport {
  /// The width of the viewport in pixels.
  pub width: u32,
  /// The height of the viewport in pixels.
  pub height: u32,
  /// The font size in pixels, used for em and rem units.
  pub font_size: f32,
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl Viewport {
  /// Creates a new viewport with the default font size.
  #[must_use]
  pub fn new(width: u32, height: u32) -> Self {
    Self::new_with_font_size(width, height, DEFAULT_FONT_SIZE)
  }

  /// Creates a new viewport with the specified font size.
  #[must_use]
  pub fn new_with_font_size(width: u32, height: u32, font_size: f32) -> Self {
    Self {
      width,
      height,
      font_size,
    }
  }
}

/// The context for the image renderer.
#[derive(Clone, Copy)]
pub struct RenderContext<'a> {
  /// The global context.
  pub global: &'a crate::core::GlobalContext,
  /// The viewport for the image renderer.
  pub viewport: Viewport,
  /// The font size in pixels, used for em and rem units.
  pub parent_font_size: f32,
}
