use taffy::{Layout, Point};

use crate::{
  effects::{BorderProperties, draw_border},
  properties::{color::Color, length_unit::SidesValue},
  rendering::FastBlendImage,
};

/// Draws debug borders around the node's layout areas.
///
/// This function draws colored rectangles to visualize the content box
/// (red) and the full layout box (green) for debugging purposes.
pub fn draw_debug_border(canvas: &mut FastBlendImage, layout: Layout) {
  let x = layout.content_box_x();
  let y = layout.content_box_y();
  let size = layout.content_box_size();

  draw_border(
    canvas,
    BorderProperties {
      width: SidesValue::SingleValue(1.0).into(),
      offset: Point { x, y },
      size,
      color: Color([255, 0, 0, 255]),
      radius: None,
    },
  );

  draw_border(
    canvas,
    BorderProperties {
      width: SidesValue::SingleValue(1.0).into(),
      offset: layout.location,
      size: layout.size,
      color: Color([0, 255, 0, 255]),
      radius: None,
    },
  );
}
