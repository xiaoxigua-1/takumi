use taffy::{AvailableSpace, geometry::Size};

use crate::{core::RenderContext, rendering::construct_text_buffer, style::ResolvedFontStyle};

/// Measures the size of image based on known dimensions and available space.
pub fn measure_image(
  image_size: Size<f32>,
  known_dimensions: Size<Option<f32>>,
  available_space: Size<AvailableSpace>,
) -> Size<f32> {
  let mut width = known_dimensions.width;
  let mut height = known_dimensions.height;

  // If both dimensions are specified, use them directly
  if let Some(width) = width
    && let Some(height) = height
  {
    return Size { width, height };
  }

  // If only one dimension is specified, calculate the other maintaining aspect ratio
  let aspect_ratio = if image_size.height != 0.0 {
    image_size.width / image_size.height
  } else {
    1.0
  };

  if width.is_some() && height.is_none() {
    height = Some(width.unwrap() / aspect_ratio);
  } else if height.is_some() && width.is_none() {
    width = Some(height.unwrap() * aspect_ratio);
  }

  // If neither dimension is specified, use intrinsic size but constrain to available space
  if width.is_none() && height.is_none() {
    width = Some(image_size.width);
    height = Some(image_size.height);
  }

  let mut final_width = width.unwrap();
  let mut final_height = height.unwrap();

  // Constrain to available space
  match available_space.width {
    AvailableSpace::Definite(max_width) => {
      if final_width > max_width {
        final_width = max_width;
        final_height = final_width / aspect_ratio;
      }
    }
    AvailableSpace::MinContent | AvailableSpace::MaxContent => {
      // Use intrinsic size for min/max content
    }
  }

  match available_space.height {
    AvailableSpace::Definite(max_height) => {
      if final_height > max_height {
        final_height = max_height;
        final_width = final_height * aspect_ratio;

        // Re-check width constraint after height adjustment
        if let AvailableSpace::Definite(max_width) = available_space.width {
          if final_width > max_width {
            final_width = max_width;
            final_height = final_width / aspect_ratio;
          }
        }
      }
    }
    AvailableSpace::MinContent | AvailableSpace::MaxContent => {
      // Use intrinsic size for min/max content
    }
  }

  Size {
    width: final_width,
    height: final_height,
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
  if text.trim().is_empty()
    || known_dimensions.width == Some(0.0)
    || known_dimensions.height == Some(0.0)
  {
    return Size {
      width: 0.0,
      height: 0.0,
    };
  }

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
    (Some(max_lines), Some(height)) => Some((max_lines as f32 * style.line_height).min(height)),
    (Some(max_lines), None) => Some(max_lines as f32 * style.line_height),
    (None, Some(height)) => Some(height),
    (None, None) => None,
  };

  let buffer = construct_text_buffer(
    text,
    style,
    context,
    Some((width_constraint, height_constraint_with_max_lines)),
  );

  let (max_run_width, total_lines) = buffer.layout_runs().fold((0.0, 0usize), |(w, lines), run| {
    (run.line_w.max(w), lines + 1)
  });

  let measured_height = total_lines as f32 * buffer.metrics().line_height;

  taffy::Size {
    // Ceiling to avoid sub-pixel getting cutoff
    width: max_run_width.ceil(),
    height: measured_height.ceil(),
  }
}
