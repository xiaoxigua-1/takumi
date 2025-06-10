use cosmic_text::{Attrs, Buffer, Family, Metrics, Shaping};
use taffy::{AvailableSpace, geometry::Size};

use crate::{context::FontContext, node::style::FontStyle};

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
      height: width / source_aspect_ratio,
    },
    (None, Some(height)) => Size {
      width: height * source_aspect_ratio,
      height,
    },
    (None, None) => image_size,
  }
}

pub fn measure_text(
  font_context: &FontContext,
  text: &str,
  font_style: &FontStyle,
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

  let height_constraint_with_max_lines = match (font_style.max_lines, height_constraint) {
    (Some(max_lines), Some(height)) => {
      Some((max_lines as f32 * font_style.line_height * font_style.font_size).min(height))
    }
    (Some(max_lines), None) => {
      Some(max_lines as f32 * font_style.line_height * font_style.font_size)
    }
    (None, Some(height)) => Some(height),
    (None, None) => None,
  };

  let metrics = Metrics::relative(font_style.font_size, font_style.line_height);
  let mut buffer = Buffer::new_empty(metrics);

  let mut attrs = Attrs::new().weight(font_style.font_weight.into());

  if let Some(font_family) = font_style.font_family.as_ref() {
    attrs = attrs.family(Family::Name(font_family));
  }

  let mut font_system = font_context.font_system.lock().unwrap();

  buffer.set_size(
    &mut font_system,
    width_constraint,
    height_constraint_with_max_lines,
  );
  buffer.set_rich_text(
    &mut font_system,
    [(text, attrs.clone())],
    &attrs,
    Shaping::Advanced,
    None,
  );

  let (width, total_lines) = buffer
    .layout_runs()
    .fold((0.0, 0usize), |(width, total_lines), run| {
      (run.line_w.max(width), total_lines + 1)
    });
  let height = total_lines as f32 * buffer.metrics().line_height;

  taffy::Size { width, height }
}
