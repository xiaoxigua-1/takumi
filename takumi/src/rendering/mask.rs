use zeno::{Fill, Mask, PathBuilder, Placement, Style};

use crate::rendering::BorderRadius;

// Returns a alpha mask for the given border radius.
pub fn create_mask(
  width: f32,
  height: f32,
  border_radius: BorderRadius,
  fill: Fill,
) -> (Vec<u8>, Placement) {
  let mut path = Vec::new();

  // CSS border-radius is clamped to half the smallest dimension
  let max_radius = (width.min(height)) / 2.0;

  let top_left = border_radius.top_left.min(max_radius);
  let top_right = border_radius.top_right.min(max_radius);
  let bottom_right = border_radius.bottom_right.min(max_radius);
  let bottom_left = border_radius.bottom_left.min(max_radius);

  // Start from top edge, after top-left radius
  path.move_to((top_left, 0.0));

  // Top edge to top-right corner
  path.line_to((width - top_right, 0.0));

  // Top-right corner using quadratic curve
  if top_right > 0.0 {
    path.quad_to((width, 0.0), (width, top_right));
  }

  // Right edge
  path.line_to((width, height - bottom_right));

  // Bottom-right corner
  if bottom_right > 0.0 {
    path.quad_to((width, height), (width - bottom_right, height));
  }

  // Bottom edge
  path.line_to((bottom_left, height));

  // Bottom-left corner
  if bottom_left > 0.0 {
    path.quad_to((0.0, height), (0.0, height - bottom_left));
  }

  // Left edge
  path.line_to((0.0, top_left));

  // Top-left corner
  if top_left > 0.0 {
    path.quad_to((0.0, 0.0), (top_left, 0.0));
  }

  path.close();

  let style = Style::Fill(fill);
  let mut mask = Mask::new(&path);

  mask.style(style);

  mask.render()
}
