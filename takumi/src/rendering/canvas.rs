//! Canvas operations and image blending for the takumi rendering system.
//!
//! This module provides performance-optimized canvas operations including
//! fast image blending and pixel manipulation operations.

use image::{Pixel, Rgba, RgbaImage};

/// A performance-optimized implementation of image blending operations.
///
/// This implementation provides faster blending by skipping pixel operations when the source color is fully transparent
/// and using direct pixel assignment when the source color is fully opaque.
///
/// Based on the implementation from [imageproc's Blend](https://docs.rs/imageproc/latest/imageproc/drawing/struct.Blend.html).
pub struct FastBlendImage(pub RgbaImage);

impl FastBlendImage {
  /// Draws a pixel onto the canvas with color alpha blending.
  pub fn draw_pixel(&mut self, x: u32, y: u32, color: Rgba<u8>) {
    if color.0[3] == 0 {
      return;
    }

    let pix = self.0.get_pixel_mut(x, y);

    if color.0[3] == 255 {
      *pix = color;
      return;
    }

    pix.blend(&color);
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
  ///
  /// This function enables rayon for parallel processing when the overlay size is greater than 50% of the image size.
  pub fn overlay_image(&mut self, image: &RgbaImage, left: u32, top: u32) {
    let target_width = image.width().min(self.width().saturating_sub(left));
    let target_height = image.height().min(self.height().saturating_sub(top));

    for y in 0..target_height {
      for x in 0..target_width {
        let pixel = *image.get_pixel(x, y);
        self.draw_pixel(x + left, y + top, pixel);
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
