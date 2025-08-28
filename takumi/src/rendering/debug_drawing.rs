use taffy::{Layout, Point};

use crate::{
  layout::style::{Color, Sides},
  rendering::{BorderProperties, BorderRadius, Canvas, draw_border},
};

/// Draws debug borders around the node's layout areas.
///
/// This function draws colored rectangles to visualize the content box
/// (red) and the full layout box (green) for debugging purposes.
pub fn draw_debug_border(canvas: &Canvas, layout: Layout) {
  let x = layout.content_box_x();
  let y = layout.content_box_y();
  let size = layout.content_box_size();

  draw_border(
    canvas,
    BorderProperties {
      width: Sides([1.0; 4]).into(),
      offset: Point { x, y },
      size,
      color: Color([255, 0, 0, 255]),
      radius: BorderRadius::zero(),
    },
  );

  draw_border(
    canvas,
    BorderProperties {
      width: Sides([1.0; 4]).into(),
      offset: layout.location,
      size: layout.size,
      color: Color([0, 255, 0, 255]),
      radius: BorderRadius::zero(),
    },
  );
}
