use image::{Rgba, RgbaImage};
use taffy::{Point, Size};

use crate::{
  layout::style::{Color, LinearGradient, RadialGradient},
  rendering::{BorderRadius, FastBlendImage},
};

/// Draws a filled rectangle with a solid color.
pub fn draw_filled_rect_color(
  canvas: &mut FastBlendImage,
  size: Size<f32>,
  offset: Point<f32>,
  color: Color,
  radius: Option<BorderRadius>,
) {
  let color: Rgba<u8> = color.into();
  let size = Size {
    width: size.width as u32,
    height: size.height as u32,
  };

  let Some(radius) = radius else {
    // Fast path: if drawing on the entire canvas, we can just replace the entire canvas with the color
    if color.0[3] == 255
      && offset.x == 0.0
      && offset.y == 0.0
      && size.width == canvas.width()
      && size.height == canvas.height()
    {
      let canvas_mut = canvas.0.as_mut();

      let canvas_len = canvas_mut.len();

      for i in (0..canvas_len).step_by(4) {
        canvas_mut[i..i + 4].copy_from_slice(&color.0);
      }

      return;
    }

    for y in 0..size.height {
      for x in 0..size.width {
        canvas.draw_pixel(x + offset.x as u32, y + offset.y as u32, color);
      }
    }

    return;
  };

  let mut image = RgbaImage::from_pixel(size.width, size.height, color);

  radius.apply_to_image(&mut image);

  canvas.overlay_image(&image, offset.x as u32, offset.y as u32);
}

/// Draws a filled rectangle with a linear gradient.
pub fn draw_filled_rect_gradient(
  canvas: &mut FastBlendImage,
  size: Size<f32>,
  offset: Point<f32>,
  gradient: &LinearGradient,
  radius: Option<BorderRadius>,
) {
  let mut gradient_image = create_gradient_image(gradient, size.width as u32, size.height as u32);

  if let Some(radius) = radius {
    radius.apply_to_image(&mut gradient_image);
  }

  canvas.overlay_image(&gradient_image, offset.x as u32, offset.y as u32);
}

/// Creates an image from a gradient.
pub fn create_gradient_image(color: &LinearGradient, width: u32, height: u32) -> RgbaImage {
  let mut ctx = color.to_draw_context(width as f32, height as f32);
  RgbaImage::from_fn(width, height, |x, y| color.at(x, y, &mut ctx).into())
}

/// Draws a filled rectangle with a radial gradient.
pub fn draw_filled_rect_radial_gradient(
  canvas: &mut FastBlendImage,
  size: Size<f32>,
  offset: Point<f32>,
  gradient: &RadialGradient,
  radius: Option<BorderRadius>,
) {
  let mut gradient_image =
    create_radial_gradient_image(gradient, size.width as u32, size.height as u32);

  if let Some(radius) = radius {
    radius.apply_to_image(&mut gradient_image);
  }

  canvas.overlay_image(&gradient_image, offset.x as u32, offset.y as u32);
}

/// Creates an image from a radial gradient.
pub fn create_radial_gradient_image(
  gradient: &RadialGradient,
  width: u32,
  height: u32,
) -> RgbaImage {
  let mut ctx = gradient.to_draw_context(width as f32, height as f32);
  RgbaImage::from_fn(width, height, |x, y| gradient.at(x, y, &mut ctx).into())
}
