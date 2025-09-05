use std::f32::consts::SQRT_2;

use taffy::{Layout, Point, Rect, Size};
use zeno::{Command, PathBuilder};

use crate::{layout::style::LengthUnit, rendering::RenderContext};

/// Represents the four corners of an image for border radius processing.
#[derive(Debug, Default, Clone, Copy)]
pub struct BorderRadius {
  /// The radius of the top-left corner
  pub top_left: f32,
  /// The radius of the top-right corner
  pub top_right: f32,
  /// The radius of the bottom-right corner
  pub bottom_right: f32,
  /// The radius of the bottom-left corner
  pub bottom_left: f32,
  /// The width of the image
  pub size: Size<f32>,
  /// The offset of the image
  pub offset: Point<f32>,
}

impl BorderRadius {
  /// Creates a new `BorderRadius` with all corners set to zero radius.
  ///
  /// # Returns
  /// A `BorderRadius` instance with no corner rounding
  pub const fn zero() -> Self {
    Self {
      top_left: 0.0,
      top_right: 0.0,
      bottom_right: 0.0,
      bottom_left: 0.0,
      size: Size::ZERO,
      offset: Point::ZERO,
    }
  }

  /// Increases all corner radii by the specified amount and adjusts size and offset accordingly.
  ///
  /// This method expands the border radius by adding the given radius value to each corner,
  /// increases the size by twice the radius (to account for the expanded corners), and
  /// adjusts the offset to center the expanded area.
  ///
  /// # Arguments
  /// * `radius` - The amount to increase each corner radius by
  ///
  /// # Returns
  /// A new `BorderRadius` instance with increased corner radii and adjusted size/offset
  pub fn grow(&self, radius: f32) -> Self {
    Self {
      top_left: (self.top_left + radius).max(0.0),
      top_right: (self.top_right + radius).max(0.0),
      bottom_right: (self.bottom_right + radius).max(0.0),
      bottom_left: (self.bottom_left + radius).max(0.0),
      size: Size {
        width: (self.size.width + radius * 2.0).max(0.0),
        height: (self.size.height + radius * 2.0).max(0.0),
      },
      offset: Point {
        x: self.offset.x - radius,
        y: self.offset.y - radius,
      },
    }
  }

  /// Offsets the radius by half of the size.
  pub fn offset_by_half(&self) -> Self {
    Self {
      offset: Point {
        x: self.offset.x - self.size.width / 2.0,
        y: self.offset.y - self.size.height / 2.0,
      },
      ..*self
    }
  }

  /// Generates path commands to create a mask representing the border radius shape.
  /// Uses cubic Bézier curves to approximate CSS border-radius quarter-circles.
  pub fn write_mask_commands(&self, path: &mut Vec<Command>) {
    // CSS border-radius uses cubic Bézier approximation of quarter circles
    // The magic number 0.552284749831 ≈ 4/3 * (√2 - 1) gives the best circular approximation
    const KAPPA: f32 = 4.0 / 3.0 * (SQRT_2 - 1.0);

    // Calculate the available space for each edge
    let top_edge_width = (self.size.width - self.top_left - self.top_right).max(0.0);
    let right_edge_height = (self.size.height - self.top_right - self.bottom_right).max(0.0);
    let bottom_edge_width = (self.size.width - self.bottom_left - self.bottom_right).max(0.0);
    let left_edge_height = (self.size.height - self.bottom_left - self.top_left).max(0.0);

    // Start at the end of the top-left radius along the top edge
    path.move_to((self.offset.x + self.top_left, self.offset.y));

    // Top edge - horizontal line from top-left corner end to top-right corner start
    if top_edge_width > 0.0 {
      path.rel_line_to((top_edge_width, 0.0));
    }

    // Top-right corner - quarter circle using cubic Bézier
    if self.top_right > 0.0 {
      let control_offset = self.top_right * KAPPA;
      path.rel_curve_to(
        (control_offset, 0.0),                             // first control point
        (self.top_right, self.top_right - control_offset), // second control point
        (self.top_right, self.top_right),                  // end point
      );
    }

    // Right edge - vertical line from top-right corner end to bottom-right corner start
    if right_edge_height > 0.0 {
      path.rel_line_to((0.0, right_edge_height));
    }

    // Bottom-right corner - quarter circle using cubic Bézier
    if self.bottom_right > 0.0 {
      let control_offset = self.bottom_right * KAPPA;
      path.rel_curve_to(
        (0.0, control_offset),                                    // first control point
        (-self.bottom_right + control_offset, self.bottom_right), // second control point
        (-self.bottom_right, self.bottom_right),                  // end point
      );
    }

    // Bottom edge - horizontal line from bottom-right corner end to bottom-left corner start
    if bottom_edge_width > 0.0 {
      path.rel_line_to((-bottom_edge_width, 0.0));
    }

    // Bottom-left corner - quarter circle using cubic Bézier
    if self.bottom_left > 0.0 {
      let control_offset = self.bottom_left * KAPPA;
      path.rel_curve_to(
        (-control_offset, 0.0),                                  // first control point
        (-self.bottom_left, -self.bottom_left + control_offset), // second control point
        (-self.bottom_left, -self.bottom_left),                  // end point
      );
    }

    // Left edge - vertical line from bottom-left corner end to top-left corner start
    if left_edge_height > 0.0 {
      path.rel_line_to((0.0, -left_edge_height));
    }

    // Top-left corner - quarter circle using cubic Bézier
    if self.top_left > 0.0 {
      let control_offset = self.top_left * KAPPA;
      path.rel_curve_to(
        (0.0, -control_offset),                           // first control point
        (self.top_left - control_offset, -self.top_left), // second control point
        (self.top_left, -self.top_left),                  // end point
      );
    }

    // Close the path
    path.close();
  }

  /// Returns true if all corners have a radius of 0.
  #[inline]
  pub fn is_zero(&self) -> bool {
    self.top_left == 0.0
      && self.top_right == 0.0
      && self.bottom_right == 0.0
      && self.bottom_left == 0.0
  }

  /// Creates a new `BorderRadius` from a `Layout` and a `Rect` of `LengthUnit`s.
  pub fn from_layout(context: &RenderContext, layout: &Layout, radius: Rect<LengthUnit>) -> Self {
    // CSS border-radius percentages: use smaller of width/height for circular corners
    let reference_size = layout.size.width.min(layout.size.height);

    Self {
      top_left: resolve_border_radius_from_percentage_css(context, radius.top, reference_size),
      top_right: resolve_border_radius_from_percentage_css(context, radius.right, reference_size),
      bottom_right: resolve_border_radius_from_percentage_css(
        context,
        radius.bottom,
        reference_size,
      ),
      bottom_left: resolve_border_radius_from_percentage_css(context, radius.left, reference_size),
      size: layout.size,
      offset: Point::ZERO,
    }
  }
}

fn resolve_border_radius_from_percentage_css(
  context: &RenderContext,
  radius: LengthUnit,
  reference_size: f32,
) -> f32 {
  radius
    .resolve_to_px(context, reference_size)
    .min(reference_size / 2.0)
}
