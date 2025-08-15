//! Viewport definitions and related types for the takumi rendering system.
//!
//! This module contains the viewport type and related context structures
//! that define the rendering area and font sizing.

/// The default font size in pixels.
pub const DEFAULT_FONT_SIZE: f32 = 16.0;

/// The default line height multiplier.
pub const DEFAULT_LINE_HEIGHT_SCALER: f32 = 1.2;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_viewport_new_defaults() {
    let v = Viewport::new(800, 600);
    assert_eq!(v.width, 800);
    assert_eq!(v.height, 600);
    assert_eq!(v.font_size, DEFAULT_FONT_SIZE);
  }

  #[test]
  fn test_viewport_new_with_font_size() {
    let v = Viewport::new_with_font_size(1024, 768, 14.0);
    assert_eq!(v.width, 1024);
    assert_eq!(v.height, 768);
    assert_eq!(v.font_size, 14.0);
  }

  #[test]
  fn test_render_context_fields() {
    let global = Box::leak(Box::new(crate::core::GlobalContext::default()));
    let rc = RenderContext {
      global,
      viewport: Viewport::new_with_font_size(800, 600, 16.0),
      parent_font_size: 16.0,
    };

    assert_eq!(rc.viewport.width, 800);
    assert_eq!(rc.viewport.height, 600);
    assert_eq!(rc.parent_font_size, 16.0);
  }
}
