use image::Rgba;
use imageproc::{drawing::draw_hollow_rect_mut, rect::Rect};
use taffy::Layout;

use crate::rendering::FastBlendImage;

/// Draws debug borders around the node's layout areas.
///
/// This function draws colored rectangles to visualize the content box
/// (red) and the full layout box (green) for debugging purposes.
pub fn draw_debug_border(canvas: &mut FastBlendImage, layout: Layout) {
  let x = layout.content_box_x();
  let y = layout.content_box_y();
  let size = layout.content_box_size();

  draw_hollow_rect_mut(
    canvas,
    Rect::at(x as i32, y as i32).of_size(size.width as u32, size.height as u32),
    Rgba([255, 0, 0, 100]),
  );

  draw_hollow_rect_mut(
    canvas,
    Rect::at(layout.location.x as i32, layout.location.y as i32)
      .of_size(layout.size.width as u32, layout.size.height as u32),
    Rgba([0, 255, 0, 100]),
  );
}
