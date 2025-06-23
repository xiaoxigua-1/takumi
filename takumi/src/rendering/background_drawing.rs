use image::RgbaImage;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use taffy::Layout;

use crate::{
  effects::{BorderRadius, apply_border_radius_antialiased},
  rendering::FastBlendImage,
  style::{ColorAt, ColorInput, Gradient},
};

/// Draws a filled rectangle on the canvas from a color input.
pub fn draw_filled_rect_from_color_input(
  canvas: &mut FastBlendImage,
  rect: Rect,
  color: &ColorInput,
) {
  match color {
    ColorInput::Color(color) => {
      draw_filled_rect_mut(canvas, rect, (*color).into());
    }
    ColorInput::Gradient(gradient) => {
      let gradient_image = create_gradient_image(gradient, rect.width(), rect.height());

      canvas.overlay_image(&gradient_image, rect.left() as u32, rect.top() as u32);
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
pub fn draw_background_color(
  color: &ColorInput,
  radius: Option<BorderRadius>,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  let rect = Rect::at(layout.location.x as i32, layout.location.y as i32)
    .of_size(layout.size.width as u32, layout.size.height as u32);

  let Some(radius) = radius else {
    draw_filled_rect_from_color_input(canvas, rect, color);
    return;
  };

  let mut image = create_image_from_color_input(color, rect.width(), rect.height());

  apply_border_radius_antialiased(&mut image, radius);

  canvas.overlay_image(&image, layout.location.x as u32, layout.location.y as u32);
}
