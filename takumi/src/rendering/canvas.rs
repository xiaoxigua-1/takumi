//! Canvas operations and image blending for the takumi rendering system.
//!
//! This module provides performance-optimized canvas operations including
//! fast image blending and pixel manipulation operations.

use image::{Pixel, Rgba, RgbaImage};

/// A performance-optimized implementation of image blending operations.
///
/// This implementation provides faster blending by skipping pixel operations when the source color is fully transparent
/// and using direct pixel assignment when the source color is fully opaque.
pub struct FastBlendImage(pub RgbaImage);

impl FastBlendImage {
  /// Draws a pixel onto the canvas with color alpha blending.
  pub fn draw_pixel(&mut self, x: u32, y: u32, color: Rgba<u8>) {
    if color.0[3] == 0 {
      return;
    }

    // image-rs blend will skip the operation if the source color is fully transparent
    self.0.get_pixel_mut(x, y).blend(&color);
  }
}

impl FastBlendImage {
  /// Creates a new FastBlendImage from an RgbaImage.
  pub fn new(image: RgbaImage) -> Self {
    Self(image)
  }

  /// Gets the width of the canvas.
  #[inline]
  pub fn width(&self) -> u32 {
    self.0.width()
  }

  /// Gets the height of the canvas.
  #[inline]
  pub fn height(&self) -> u32 {
    self.0.height()
  }

  /// Draws an image onto the canvas with an offset.
  pub fn overlay_image(&mut self, image: &RgbaImage, left: i32, top: i32) {
    let drawable_width = if left < 0 {
      image.width().saturating_sub(left.saturating_neg() as u32)
    } else {
      image.width().min(self.width().saturating_sub(left as u32))
    };

    let drawable_height = if top < 0 {
      image.height().saturating_sub(top.saturating_neg() as u32)
    } else {
      image.height().min(self.height().saturating_sub(top as u32))
    };

    if drawable_width == 0 || drawable_height == 0 {
      return;
    }

    let overlay_x = if left < 0 {
      left.saturating_neg() as u32
    } else {
      0
    };
    let overlay_y = if top < 0 {
      top.saturating_neg() as u32
    } else {
      0
    };

    let draw_x = left.max(0) as u32;
    let draw_y = top.max(0) as u32;

    for y in 0..drawable_height {
      for x in 0..drawable_width {
        let pixel = *image.get_pixel(x + overlay_x, y + overlay_y);
        self.draw_pixel(x + draw_x, y + draw_y, pixel);
      }
    }
  }

  /// Consumes the FastBlendImage and returns the underlying RgbaImage.
  pub fn into_image(self) -> RgbaImage {
    self.0
  }

  /// Gets a reference to the underlying RgbaImage.
  pub fn as_image(&self) -> &RgbaImage {
    &self.0
  }

  /// Gets a mutable reference to the underlying RgbaImage.
  pub fn as_image_mut(&mut self) -> &mut RgbaImage {
    &mut self.0
  }
}
