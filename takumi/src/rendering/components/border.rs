use std::sync::Arc;

use image::{Rgba, RgbaImage};
use taffy::{Layout, Point, Size};
use zeno::Mask;

use crate::{
  layout::style::{Color, Style},
  rendering::{BorderRadius, Canvas, RenderContext},
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
  if border.width.left == 0.0
    && border.width.right == 0.0
    && border.width.top == 0.0
    && border.width.bottom == 0.0
  {
    return;
  }

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
    canvas.fill_color(
      Point {
        x: border.offset.x as i32,
        y: border.offset.y as i32,
      },
      Size {
        width: border.size.width as u32,
        height: border.width.top as u32,
      },
      border.color,
      BorderRadius::zero(),
    );
  }

  // Bottom border
  if border.width.bottom > 0.0 {
    canvas.fill_color(
      Point {
        x: border.offset.x as i32,
        y: (border.offset.y + border.size.height - border.width.bottom) as i32,
      },
      Size {
        width: border.size.width as u32,
        height: border.width.bottom as u32,
      },
      border.color,
      BorderRadius::zero(),
    );
  }

  // Left border (excluding corners already drawn by top/bottom)
  if border.width.left > 0.0 {
    canvas.fill_color(
      Point {
        x: border.offset.x as i32,
        y: (border.offset.y + border.width.top) as i32,
      },
      Size {
        width: border.width.left as u32,
        height: (border.size.height - border.width.top - border.width.bottom) as u32,
      },
      border.color,
      BorderRadius::zero(),
    );
  }

  // Right border (excluding corners already drawn by top/bottom)
  if border.width.right > 0.0 {
    canvas.fill_color(
      Point {
        x: (border.offset.x + border.size.width - border.width.right) as i32,
        y: (border.offset.y + border.width.top) as i32,
      },
      Size {
        width: border.width.right as u32,
        height: (border.size.height - border.width.top - border.width.bottom) as u32,
      },
      border.color,
      BorderRadius::zero(),
    );
  }
}

/// Draws a rounded border with border radius.
fn draw_rounded_border(canvas: &Canvas, border: BorderProperties) {
  // Create a temporary image filled with border color
  let mut border_image = RgbaImage::from_pixel(
    border.size.width as u32,
    border.size.height as u32,
    border.color.into(),
  );

  let mut paths = Vec::new();

  border.radius.write_mask_commands(&mut paths);

  let avg_border_width =
    (border.width.left + border.width.right + border.width.top + border.width.bottom) / 4.0;

  let inner_border_radius = BorderRadius {
    offset: Point {
      x: border.width.left,
      y: border.width.top,
    },
    size: Size {
      width: border.size.width - border.width.left - border.width.right,
      height: border.size.height - border.width.top - border.width.bottom,
    },
    ..border.radius.grow(avg_border_width)
  };

  inner_border_radius.write_mask_commands(&mut paths);

  let (mask, placement) = Mask::new(&paths).render();

  let mut i = 0;

  for y in 0..border.size.height as i32 {
    for x in 0..border.size.width as i32 {
      let alpha = mask[i];

      i += 1;

      if alpha == 0 {
        continue;
      }

      let x = x + placement.left;
      let y = y + placement.top;

      let pixel = Rgba([
        border.color.0[0],
        border.color.0[1],
        border.color.0[2],
        alpha,
      ]);

      border_image.put_pixel(x as u32, y as u32, pixel);
    }
  }

  // Overlay the border image onto the canvas
  canvas.overlay_image(
    Arc::new(border_image),
    Point {
      x: border.offset.x as i32,
      y: border.offset.y as i32,
    },
    BorderRadius::zero(),
  );
}
