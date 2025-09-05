use taffy::{Layout, Point, Size};
use zeno::{Fill, Mask};

use crate::{
  layout::style::{Affine, Color, Style},
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
  /// The transform of the border.
  pub transform: Affine,
}

impl BorderProperties {
  /// Creates a new `BorderProperties` from a `Layout` and a `Style`.
  pub fn from_layout(context: &RenderContext, layout: &Layout, style: &Style) -> Self {
    Self {
      width: layout.border,
      offset: layout.location,
      size: layout.size,
      color: style
        .inheritable_style
        .border_color
        .unwrap_or_else(Color::black),
      radius: style.create_border_radius(layout, context),
      transform: context.transform,
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

  let mut paths = Vec::new();

  border.radius.write_mask_commands(&mut paths);

  let avg_border_width =
    (border.width.left + border.width.right + border.width.top + border.width.bottom) / 4.0;

  let inner_border_radius = border.radius.grow(-avg_border_width);

  inner_border_radius.write_mask_commands(&mut paths);

  let mut mask = Mask::new(&paths);

  mask.style(Fill::EvenOdd);

  let (mask, mut placement) = mask.render();

  placement.left += border.offset.x as i32;
  placement.top += border.offset.y as i32;

  canvas.draw_mask(mask, placement, border.color, None);
}
