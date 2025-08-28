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

  /// Generates path commands to create a mask representing the border radius shape.
  ///
  /// This method creates a vector path that defines the outline of a rectangle with
  /// rounded corners based on the border radius values. The path is constructed by:
  /// 1. Starting at the top-left corner (after the radius)
  /// 2. Drawing straight edges between corners
  /// 3. Adding quadratic curves for each corner with non-zero radius
  /// 4. Closing the path
  ///
  /// The resulting path can be used for masking operations to create rounded corners.
  ///
  /// # Arguments
  /// * `path` - The vector to append the path commands to
  pub fn write_mask_commands(&self, path: &mut Vec<Command>) {
    // Calculate the straight edge lengths (total size minus corner radii)
    let top_edge_width = self.size.width - self.top_left - self.top_right;
    let right_edge_height = self.size.height - self.top_right - self.bottom_right;
    let bottom_edge_width = self.size.width - self.bottom_left - self.bottom_right;
    let left_edge_height = self.size.height - self.bottom_left - self.top_left;

    // Start at top-left corner (after the radius)
    path.move_to((self.offset.x + self.top_left, self.offset.y));

    // Top edge to top-right corner
    path.rel_line_to((top_edge_width, 0.0));

    // Top-right corner using quadratic curve
    if self.top_right > 0.0 {
      path.rel_quad_to(
        (self.top_right, 0.0),            // control point: corner of rectangle
        (self.top_right, self.top_right), // end point: down by radius
      );
    }

    // Right edge
    path.rel_line_to((0.0, right_edge_height));

    // Bottom-right corner
    if self.bottom_right > 0.0 {
      path.rel_quad_to(
        (0.0, self.bottom_right), // control point: corner of rectangle
        (-self.bottom_right, self.bottom_right), // end point: left by radius
      );
    }

    // Bottom edge
    path.rel_line_to((-bottom_edge_width, 0.0));

    // Bottom-left corner
    if self.bottom_left > 0.0 {
      path.rel_quad_to(
        (-self.bottom_left, 0.0),               // control point: corner of rectangle
        (-self.bottom_left, -self.bottom_left), // end point: up by radius
      );
    }

    // Left edge
    path.rel_line_to((0.0, -left_edge_height));

    // Top-left corner
    if self.top_left > 0.0 {
      path.rel_quad_to(
        (0.0, -self.top_left),           // control point: corner of rectangle
        (self.top_left, -self.top_left), // end point: right by radius
      );
    }

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
  match radius {
    LengthUnit::Percentage(value) => value * reference_size / 100.0,
    rest => rest.resolve_to_px(context).min(reference_size / 2.0),
  }
}
