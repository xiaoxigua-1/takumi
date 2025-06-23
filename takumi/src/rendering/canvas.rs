//! Canvas operations and image blending for the takumi rendering system.
//!
//! This module provides performance-optimized canvas operations including
//! fast image blending and pixel manipulation operations.

use image::{Pixel, Rgba, RgbaImage};
use imageproc::drawing::Canvas;
use rayon::prelude::*;

/// A performance-optimized implementation of image blending operations.
///
/// This implementation provides faster blending by skipping pixel operations when the source color is fully transparent
/// and using direct pixel assignment when the source color is fully opaque.
///
/// Based on the implementation from [imageproc's Blend](https://docs.rs/imageproc/latest/imageproc/drawing/struct.Blend.html).
pub struct FastBlendImage(pub RgbaImage);

impl Canvas for FastBlendImage {
  type Pixel = Rgba<u8>;

  fn dimensions(&self) -> (u32, u32) {
    self.0.dimensions()
  }

  fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
    *self.0.get_pixel(x, y)
  }

  fn draw_pixel(&mut self, x: u32, y: u32, color: Self::Pixel) {
    if color.0[3] == 0 {
      return;
    }

    if color.0[3] == 255 {
      self.0.put_pixel(x, y, color);
      return;
    }

    let mut pix = *self.0.get_pixel(x, y);

    pix.blend(&color);

    self.0.put_pixel(x, y, pix);
  }
}

impl FastBlendImage {
  /// Creates a new FastBlendImage from an RgbaImage.
  pub fn new(image: RgbaImage) -> Self {
    Self(image)
  }

  /// Gets the width of the canvas.
  pub fn width(&self) -> u32 {
    self.0.width()
  }

  /// Gets the height of the canvas.
  pub fn height(&self) -> u32 {
    self.0.height()
  }

  /// Draws an image onto the canvas with an offset.
  ///
  /// This function enables rayon for parallel processing when the overlay size is greater than 50% of the image size.
  pub fn overlay_image(&mut self, image: &RgbaImage, left: u32, top: u32) {
    let target_width = image.width().min(self.width().saturating_sub(left));
    let target_height = image.height().min(self.height().saturating_sub(top));

    let overlay_size_percentage =
      (target_width * target_height) as f32 / (image.width() * image.height()) as f32;

    if overlay_size_percentage < 0.5 {
      for y in 0..target_height {
        for x in 0..target_width {
          let pixel = *image.get_pixel(x, y);
          self.draw_pixel(x + left, y + top, pixel);
        }
      }

      return;
    }

    self.0.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
      if x < left || y < top || x >= left + target_width || y >= top + target_height {
        return;
      }

      let image_pixel = *image.get_pixel(x - left, y - top);

      if image_pixel.0[3] == 0 {
        return;
      }

      if image_pixel.0[3] == 255 {
        *pixel = image_pixel;
        return;
      }

      pixel.blend(&image_pixel);
    });
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
