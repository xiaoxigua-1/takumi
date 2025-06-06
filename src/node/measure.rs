use std::sync::Mutex;

use cosmic_text::{Attrs, Buffer, Metrics, Shaping};
use float_ord::FloatOrd;
use lru::LruCache;
use taffy::{AvailableSpace, geometry::Size};

use crate::{
  context::Context,
  node::{
    draw::ImageState,
    properties::{ImageProperties, TextProperties},
  },
};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TextMeasureCacheKey {
  pub font_size: FloatOrd<f32>,
  pub line_height: FloatOrd<f32>,
  pub width_constraint: Option<FloatOrd<f32>>,
  pub height_constraint: Option<FloatOrd<f32>>,
  pub content: String,
}

pub type TextMeasureCache = Mutex<LruCache<TextMeasureCacheKey, Size<f32>>>;

pub fn measure_image(
  context: &Context,
  props: &ImageProperties,
  known_dimensions: Size<Option<f32>>,
  available_space: Size<AvailableSpace>,
) -> Size<f32> {
  let mut lock = context.image_fetch_cache.lock().unwrap();
  let Some(ImageState::Fetched(image)) = lock.get(&props.src) else {
    return Size::ZERO;
  };

  let source_width = image.width() as f32;
  let source_height = image.height() as f32;

  let source_aspect_ratio = source_width / source_height;

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
    (None, None) => Size {
      width: source_width,
      height: source_height,
    },
  }
}

pub fn measure_text(
  context: &Context,
  props: &TextProperties,
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

  let cache_key = TextMeasureCacheKey {
    font_size: FloatOrd(props.font_size),
    line_height: FloatOrd(props.line_height),
    width_constraint: width_constraint.map(FloatOrd),
    height_constraint: height_constraint.map(FloatOrd),
    content: props.content.clone(),
  };

  let mut lock = context.text_measure_cache.lock().unwrap();
  if let Some(size) = lock.get(&cache_key) {
    return *size;
  }

  drop(lock);

  let mut font_system = context.font_system.lock().unwrap();

  let metrics = Metrics::relative(props.font_size, props.line_height);
  let mut buffer = Buffer::new(&mut font_system, metrics);

  buffer.set_size(&mut font_system, width_constraint, height_constraint);

  let attrs = Attrs::new().weight(props.font_weight.into());

  buffer.set_text(&mut font_system, &props.content, &attrs, Shaping::Advanced);

  buffer.shape_until_scroll(&mut font_system, false);

  let (width, total_lines) = buffer
    .layout_runs()
    .fold((0.0, 0usize), |(width, total_lines), run| {
      (run.line_w.max(width), total_lines + 1)
    });
  let height = total_lines as f32 * buffer.metrics().line_height;

  let size = taffy::Size { width, height };

  let mut lock = context.text_measure_cache.lock().unwrap();
  lock.put(cache_key, size);

  size
}
