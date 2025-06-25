use taffy::{AvailableSpace, geometry::Size};

use crate::{core::RenderContext, rendering::construct_text_buffer, style::ResolvedFontStyle};

/// Measures the size of an image based on available space and known dimensions.
///
/// This function handles aspect ratio preservation and respects both explicit
/// dimensions and available space constraints.
pub fn measure_image(
  image_size: Size<f32>,
  known_dimensions: Size<Option<f32>>,
  available_space: Size<AvailableSpace>,
) -> Size<f32> {
  let source_aspect_ratio = image_size.width / image_size.height;

  let hint_width = known_dimensions.width.or({
    if let AvailableSpace::Definite(available_width) = available_space.width {
      Some(available_width)
    } else {
      None
    }
  });

  let hint_height = known_dimensions.height.or({
    if let AvailableSpace::Definite(available_height) = available_space.height {
      Some(available_height)
    } else {
      None
    }
  });

  match (hint_width, hint_height) {
    (Some(width), Some(height)) => Size { width, height },
    (Some(width), None) => Size {
      width,
      height: (width / source_aspect_ratio).ceil(),
    },
    (None, Some(height)) => Size {
      width: (height * source_aspect_ratio).ceil(),
      height,
    },
    (None, None) => image_size,
  }
}

/// Measures the size of text based on font style and available space.
///
/// This function handles text wrapping, line height, and respects both explicit
/// dimensions and available space constraints.
pub fn measure_text(
  context: &RenderContext,
  text: &str,
  style: &ResolvedFontStyle,
  known_dimensions: Size<Option<f32>>,
  available_space: Size<AvailableSpace>,
) -> Size<f32> {
  let width_constraint = known_dimensions.width.or(match available_space.width {
    AvailableSpace::MinContent => Some(0.0),
    AvailableSpace::MaxContent => None,
    AvailableSpace::Definite(width) => Some(width),
  });

  let height_constraint = known_dimensions.height.or(match available_space.height {
    AvailableSpace::MinContent => Some(0.0),
    AvailableSpace::MaxContent => None,
    AvailableSpace::Definite(height) => Some(height),
  });

  let height_constraint_with_max_lines = match (style.line_clamp, height_constraint) {
    (Some(max_lines), Some(height)) => {
      Some((max_lines as f32 * style.line_height * style.font_size).min(height))
    }
    (Some(max_lines), None) => Some(max_lines as f32 * style.line_height * style.font_size),
    (None, Some(height)) => Some(height),
    (None, None) => None,
  };

  let mut buffer = construct_text_buffer(text, style, context);

  let mut font_system = context.global.font_context.font_system.lock().unwrap();

  buffer.set_size(
    &mut font_system,
    width_constraint,
    height_constraint_with_max_lines,
  );

  let (width, total_lines) = buffer
    .layout_runs()
    .fold((0.0, 0usize), |(width, total_lines), run| {
      (run.line_w.max(width), total_lines + 1)
    });
  let height = total_lines as f32 * buffer.metrics().line_height;

  taffy::Size {
    width: width.ceil(),
    height: height.ceil(),
  }
}
