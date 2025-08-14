use image::RgbaImage;
use taffy::{Point, Size};

use crate::{
  effects::{BorderRadius, apply_border_radius_antialiased},
  properties::{color::Color, linear_gradient::LinearGradient},
  rendering::FastBlendImage,
};

/// Draws a filled rectangle with a solid color.
pub fn draw_filled_rect_color(
  canvas: &mut FastBlendImage,
  size: Size<f32>,
  offset: Point<f32>,
  color: Color,
  radius: Option<BorderRadius>,
) {
  let color = color.into();

  let Some(radius) = radius else {
    for y in (offset.y as u32)..(size.height + offset.y) as u32 {
      for x in (offset.x as u32)..(size.width + offset.x) as u32 {
        canvas.draw_pixel(x, y, color);
      }
    }

    return;
  };

  let mut image = RgbaImage::from_pixel(size.width as u32, size.height as u32, color);

  apply_border_radius_antialiased(&mut image, radius);

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
    apply_border_radius_antialiased(&mut gradient_image, radius);
  }

  canvas.overlay_image(&gradient_image, offset.x as u32, offset.y as u32);
}

/// Creates an image from a gradient.
pub fn create_gradient_image(color: &LinearGradient, width: u32, height: u32) -> RgbaImage {
  let stops = color.resolve_stops();
  RgbaImage::from_par_fn(width, height, |x, y| {
    color.at(width as f32, height as f32, x, y, &stops).into()
  })
}
