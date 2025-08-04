use image::RgbaImage;
use taffy::{Layout, Point, Size};

use crate::{
  effects::{BorderRadius, apply_border_radius_antialiased},
  rendering::FastBlendImage,
  style::{ColorAt, ColorInput, Gradient},
};

/// Draws a filled rectangle on the canvas from a color input.
pub fn draw_filled_rect_from_color_input(
  canvas: &mut FastBlendImage,
  size: Size<f32>,
  offset: Point<f32>,
  color: &ColorInput,
) {
  match color {
    ColorInput::Color(color) => {
      let rgba = (*color).into();

      for y in (offset.y as u32)..(size.height + offset.y) as u32 {
        for x in (offset.x as u32)..(size.width + offset.x) as u32 {
          canvas.draw_pixel(x, y, rgba);
        }
      }
    }
    ColorInput::Gradient(gradient) => {
      let gradient_image = create_gradient_image(gradient, size.width as u32, size.height as u32);

      canvas.overlay_image(&gradient_image, offset.x as u32, offset.y as u32);
    }
  }
}

/// Creates an image from a gradient.
pub fn create_gradient_image(color: &Gradient, width: u32, height: u32) -> RgbaImage {
  RgbaImage::from_par_fn(width, height, |x, y| {
    color.at(width as f32, height as f32, x, y).into()
  })
}

/// Creates an image from a color input.
pub fn create_image_from_color_input(color: &ColorInput, width: u32, height: u32) -> RgbaImage {
  match color {
    ColorInput::Color(color) => {
      let color = *color;

      RgbaImage::from_pixel(width, height, color.into())
    }
    ColorInput::Gradient(gradient) => create_gradient_image(gradient, width, height),
  }
}

/// Draws a solid color background on the canvas.
pub fn draw_background(
  color: &ColorInput,
  radius: Option<BorderRadius>,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  let Some(radius) = radius else {
    return draw_filled_rect_from_color_input(canvas, layout.size, layout.location, color);
  };

  let mut image =
    create_image_from_color_input(color, layout.size.width as u32, layout.size.height as u32);

  apply_border_radius_antialiased(&mut image, radius);

  canvas.overlay_image(&image, layout.location.x as u32, layout.location.y as u32);
}
