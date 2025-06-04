use imageproc::drawing::text_size;
use taffy::{AvailableSpace, geometry::Size};

use crate::{
  context::Context,
  node::{
    draw::ImageState,
    properties::{ImageProperties, TextProperties},
  },
};

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
  _available_space: Size<AvailableSpace>,
) -> Size<f32> {
  let font = props.font(context);

  let (width, height) = text_size(props.font_size, &font, &props.content);

  Size {
    width: width as f32,
    height: height as f32 * props.line_height,
  }
}
