use image::{Rgba, RgbaImage};
use imageproc::drawing::Blend;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use taffy::{Layout, Point, Size};

use crate::node::draw::draw_image_overlay_fast;
use crate::node::style::Style;

/// Draws borders around the node with optional border radius.
///
/// This function draws borders with specified size and color. If border_radius is specified,
/// it creates a rounded border using a custom drawing approach.
///
/// # Arguments
/// * `style` - The style containing border properties
/// * `canvas` - The canvas to draw on
/// * `layout` - The layout information for positioning
pub fn draw_border(style: &Style, canvas: &mut Blend<RgbaImage>, layout: Layout) {
  if layout.border.top == 0.0
    && layout.border.right == 0.0
    && layout.border.bottom == 0.0
    && layout.border.left == 0.0
  {
    return;
  }

  let border_color = style.inheritable_style.border_color.unwrap_or_default();
  let border_radius = style.inheritable_style.border_radius;

  if let Some(radius) = border_radius {
    // Draw rounded border using a different approach
    draw_rounded_border(
      canvas,
      layout.location,
      layout.size,
      layout.border,
      border_color.into(),
      radius,
    );
  } else {
    // Draw regular rectangular border
    draw_rectangular_border(
      canvas,
      layout.location,
      layout.size,
      layout.border,
      border_color.into(),
    );
  }
}

/// Draws a rectangular border without rounded corners.
fn draw_rectangular_border(
  canvas: &mut Blend<RgbaImage>,
  location: Point<f32>,
  size: Size<f32>,
  border_rect: taffy::Rect<f32>,
  color: Rgba<u8>,
) {
  // Top border
  if border_rect.top > 0.0 {
    draw_filled_rect_mut(
      canvas,
      Rect::at(location.x as i32, location.y as i32)
        .of_size(size.width as u32, border_rect.top as u32),
      color,
    );
  }

  // Bottom border
  if border_rect.bottom > 0.0 {
    draw_filled_rect_mut(
      canvas,
      Rect::at(
        location.x as i32,
        location.y as i32 + size.height as i32 - border_rect.bottom as i32,
      )
      .of_size(size.width as u32, border_rect.bottom as u32),
      color,
    );
  }

  // Left border (excluding corners already drawn by top/bottom)
  if border_rect.left > 0.0 {
    draw_filled_rect_mut(
      canvas,
      Rect::at(
        location.x as i32,
        location.y as i32 + border_rect.top as i32,
      )
      .of_size(
        border_rect.left as u32,
        (size.height - border_rect.top - border_rect.bottom) as u32,
      ),
      color,
    );
  }

  // Right border (excluding corners already drawn by top/bottom)
  if border_rect.right > 0.0 {
    draw_filled_rect_mut(
      canvas,
      Rect::at(
        location.x as i32 + size.width as i32 - border_rect.right as i32,
        location.y as i32 + border_rect.top as i32,
      )
      .of_size(
        border_rect.right as u32,
        (size.height - border_rect.top - border_rect.bottom) as u32,
      ),
      color,
    );
  }
}

/// Draws a rounded border with border radius.
fn draw_rounded_border(
  canvas: &mut Blend<RgbaImage>,
  location: Point<f32>,
  size: Size<f32>,
  border_rect: taffy::Rect<f32>,
  color: Rgba<u8>,
  radius: f32,
) {
  if border_rect.left == 0.0
    && border_rect.right == 0.0
    && border_rect.top == 0.0
    && border_rect.bottom == 0.0
  {
    return;
  }

  // Create a temporary image for the border
  let mut border_image = RgbaImage::new(size.width as u32, size.height as u32);

  // Fill with transparent
  for pixel in border_image.pixels_mut() {
    *pixel = Rgba([0, 0, 0, 0]);
  }

  // Calculate inner bounds (content area)
  let inner_left = border_rect.left as u32;
  let inner_right = size.width as u32 - border_rect.right as u32;
  let inner_top = border_rect.top as u32;
  let inner_bottom = size.height as u32 - border_rect.bottom as u32;

  // Calculate inner radius (outer radius minus average border width, clamped to 0)
  let avg_border_width =
    (border_rect.left + border_rect.right + border_rect.top + border_rect.bottom) / 4.0;
  let inner_radius = (radius - avg_border_width).max(0.0);

  // Create the border shape by filling between outer and inner rounded rectangles
  for py in 0..size.height as u32 {
    for px in 0..size.width as u32 {
      let is_inside_outer = is_inside_rounded_rect(
        px as f32,
        py as f32,
        0.0,
        0.0,
        size.width,
        size.height,
        radius,
      );

      let is_inside_inner = if inner_left < inner_right && inner_top < inner_bottom {
        is_inside_rounded_rect(
          px as f32,
          py as f32,
          inner_left as f32,
          inner_top as f32,
          (inner_right - inner_left) as f32,
          (inner_bottom - inner_top) as f32,
          inner_radius,
        )
      } else {
        false // No inner area if border is too thick
      };

      // Pixel is part of border if it's inside outer but outside inner
      if is_inside_outer && !is_inside_inner {
        border_image.put_pixel(px, py, color);
      }
    }
  }

  // Overlay the border image onto the canvas
  draw_image_overlay_fast(canvas, &border_image, location.x as u32, location.y as u32);
}

/// Check if a point is inside a rounded rectangle
fn is_inside_rounded_rect(
  px: f32,
  py: f32,
  rect_x: f32,
  rect_y: f32,
  rect_width: f32,
  rect_height: f32,
  radius: f32,
) -> bool {
  // Translate point to rectangle's coordinate system
  let x = px - rect_x;
  let y = py - rect_y;

  // Check if point is outside the rectangle bounds
  if x < 0.0 || x >= rect_width || y < 0.0 || y >= rect_height {
    return false;
  }

  let effective_radius = radius.min(rect_width / 2.0).min(rect_height / 2.0);

  // If radius is 0, it's just a regular rectangle
  if effective_radius <= 0.0 {
    return true;
  }

  // Check corner regions
  let corner_x;
  let corner_y;

  if x < effective_radius && y < effective_radius {
    // Top-left corner
    corner_x = effective_radius;
    corner_y = effective_radius;
  } else if x >= rect_width - effective_radius && y < effective_radius {
    // Top-right corner
    corner_x = rect_width - effective_radius;
    corner_y = effective_radius;
  } else if x < effective_radius && y >= rect_height - effective_radius {
    // Bottom-left corner
    corner_x = effective_radius;
    corner_y = rect_height - effective_radius;
  } else if x >= rect_width - effective_radius && y >= rect_height - effective_radius {
    // Bottom-right corner
    corner_x = rect_width - effective_radius;
    corner_y = rect_height - effective_radius;
  } else {
    // Point is in non-corner region, definitely inside
    return true;
  }

  // Check if point is inside the corner circle
  let dx = x - corner_x;
  let dy = y - corner_y;
  dx * dx + dy * dy <= effective_radius * effective_radius
}
