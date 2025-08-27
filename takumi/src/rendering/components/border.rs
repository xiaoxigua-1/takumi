use image::{Rgba, RgbaImage};
use taffy::{Layout, Point, Size};

use crate::{
  layout::style::{Color, Style},
  rendering::{BorderRadius, Canvas, RenderContext, draw_filled_rect_color},
};

/// Represents the properties of a border.
#[derive(Debug, Clone)]
pub struct BorderProperties {
  /// The width of the border.
  pub width: taffy::Rect<f32>,
  /// The offset of the border.
  pub offset: Point<f32>,
  /// The size of the border.
  pub size: Size<f32>,
  /// The color of the border.
  pub color: Color,
  /// The radius of the border.
  pub radius: BorderRadius,
}

impl BorderProperties {
  /// Creates a new `BorderProperties` from a `Layout` and a `Style`.
  pub fn from_layout(context: &RenderContext, layout: &Layout, style: &Style) -> Self {
    Self {
      width: layout.border,
      offset: layout.location,
      size: layout.size,
      color: style.inheritable_style.border_color.unwrap_or_default(),
      radius: style.create_border_radius(layout, context),
    }
  }
}

/// Draws borders around the node with optional border radius.
///
/// This function draws borders with specified size and color. If border_radius is specified,
/// it creates a rounded border using a custom drawing approach.
pub fn draw_border(canvas: &Canvas, border: BorderProperties) {
  if !border.radius.is_zero() {
    draw_rounded_border(canvas, border);
  } else {
    draw_rectangular_border(canvas, border);
  }
}

/// Draws a rectangular border without rounded corners.
fn draw_rectangular_border(canvas: &Canvas, border: BorderProperties) {
  // Top border
  if border.width.top > 0.0 {
    draw_filled_rect_color(
      canvas,
      Size {
        width: border.size.width,
        height: border.width.top,
      },
      Point {
        x: border.offset.x,
        y: border.offset.y,
      },
      border.color,
      BorderRadius::default(),
    );
  }

  // Bottom border
  if border.width.bottom > 0.0 {
    draw_filled_rect_color(
      canvas,
      Size {
        width: border.size.width,
        height: border.width.bottom,
      },
      Point {
        x: border.offset.x,
        y: border.offset.y + border.size.height - border.width.bottom,
      },
      border.color,
      BorderRadius::default(),
    );
  }

  // Left border (excluding corners already drawn by top/bottom)
  if border.width.left > 0.0 {
    draw_filled_rect_color(
      canvas,
      Size {
        width: border.width.left,
        height: border.size.height - border.width.top - border.width.bottom,
      },
      Point {
        x: border.offset.x,
        y: border.offset.y + border.width.top,
      },
      border.color,
      BorderRadius::default(),
    );
  }

  // Right border (excluding corners already drawn by top/bottom)
  if border.width.right > 0.0 {
    draw_filled_rect_color(
      canvas,
      Size {
        width: border.width.right,
        height: border.size.height - border.width.top - border.width.bottom,
      },
      Point {
        x: border.offset.x + border.size.width - border.width.right,
        y: border.offset.y + border.width.top,
      },
      border.color,
      BorderRadius::default(),
    );
  }
}

/// Draws a rounded border with border radius.
fn draw_rounded_border(canvas: &mut FastBlendImage, border: BorderProperties) {
  if border.width.left == 0.0
    && border.width.right == 0.0
    && border.width.top == 0.0
    && border.width.bottom == 0.0
  {
    return;
  }

  // Create a temporary image filled with border color
  let mut border_image = RgbaImage::from_pixel(
    border.size.width as u32,
    border.size.height as u32,
    border.color.into(),
  );

  // Apply antialiased border radius to the outer edge
  border.radius.apply_to_image(&mut border_image);

  // Calculate inner bounds (content area)
  let inner_left = border.width.left as u32;
  let inner_right = border.size.width as u32 - border.width.right as u32;
  let inner_top = border.width.top as u32;
  let inner_bottom = border.size.height as u32 - border.width.bottom as u32;

  // Calculate inner radius (outer radius minus average border width, clamped to 0)
  let avg_border_width =
    (border.width.left + border.width.right + border.width.top + border.width.bottom) / 4.0;
  let inner_radius = BorderRadius {
    top_left: (border.radius.top_left - avg_border_width).max(0.0),
    top_right: (border.radius.top_right - avg_border_width).max(0.0),
    bottom_right: (border.radius.bottom_right - avg_border_width).max(0.0),
    bottom_left: (border.radius.bottom_left - avg_border_width).max(0.0),
  };

  // Cut out the inner area if there's space for content
  if inner_left < inner_right && inner_top < inner_bottom {
    let inner_width = inner_right - inner_left;
    let inner_height = inner_bottom - inner_top;

    // Create inner cutout with antialiased border radius
    let mut inner_image =
      RgbaImage::from_pixel(inner_width, inner_height, Rgba([255, 255, 255, 255]));
    inner_radius.apply_to_image(&mut inner_image);

    // Cut out the inner area while preserving antialiasing from inner border
    let inner_stride = inner_width as usize * 4;
    let border_stride = border.size.width as usize * 4;

    for py in 0..inner_height {
      let inner_row_start = py as usize * inner_stride;
      let border_row_start = (py + inner_top) as usize * border_stride + inner_left as usize * 4;

      let inner_slice = &inner_image.as_raw()[inner_row_start..inner_row_start + inner_stride];

      for px in 0..inner_width {
        let inner_alpha_idx = px as usize * 4 + 3;
        let inner_alpha = inner_slice[inner_alpha_idx];

        // Use inverted alpha for cutting out - where inner has alpha, we remove border
        let cutout_alpha = 255 - inner_alpha;
        if cutout_alpha < 255 {
          let border_pixel_idx = border_row_start + px as usize * 4;
          let border_slice = border_image.as_mut();

          // Blend the cutout with existing border color, preserving border's antialiasing

          let current_alpha = border_slice[border_pixel_idx + 3];
          let new_alpha = ((current_alpha as u32 * cutout_alpha as u32) / 255) as u8;
          border_slice[border_pixel_idx + 3] = new_alpha;
        }
      }
    }
  }

  // Overlay the border image onto the canvas
  canvas.overlay_image(
    &border_image,
    border.offset.x as i32,
    border.offset.y as i32,
  );
}
