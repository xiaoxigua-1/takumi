use taffy::{Layout, Rect};

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
  pub width: u32,
  /// The height of the image
  pub height: u32,
}

impl BorderRadius {
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
      width: layout.size.width as u32,
      height: layout.size.height as u32,
    }
  }

  /// Offsets the border radius by a given number of pixels.
  pub fn offset_px(&mut self, offset: f32) {
    self.top_left += offset;
    self.top_right += offset;
    self.bottom_right += offset;
    self.bottom_left += offset;
  }
}

fn resolve_border_radius_from_percentage_css(
  context: &RenderContext,
  radius: LengthUnit,
  reference_size: f32,
) -> f32 {
  match radius {
    LengthUnit::Percentage(value) => value * reference_size / 100.0,
    rest => rest.resolve_to_px(context),
  }
}
