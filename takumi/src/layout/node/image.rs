//! Image node implementation for the takumi layout system.
//!
//! This module contains the ImageNode struct which is used to render
//! image content with support for async loading and caching.

use serde::{Deserialize, Serialize};
use taffy::{AvailableSpace, Layout, Size};

#[cfg(feature = "image_data_uri")]
use crate::resources::image::ImageResult;
use crate::{
  GlobalContext,
  layout::{node::Node, style::Style},
  rendering::{FastBlendImage, RenderContext, draw_image},
  resources::image::{ImageResourceError, ImageSource, is_svg},
};

/// A node that renders image content.
///
/// Image nodes display images loaded from URLs or file paths,
/// with support for async loading and caching.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageNode {
  /// The styling properties for this image node
  #[serde(default)]
  pub style: Style,
  /// The source URL or path to the image
  pub src: String,
  /// The width of the image
  pub width: Option<f32>,
  /// The height of the image
  pub height: Option<f32>,
}

impl<Nodes: Node<Nodes>> Node<Nodes> for ImageNode {
  fn get_style(&self) -> &Style {
    &self.style
  }

  fn get_style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

  fn measure(
    &self,
    context: &RenderContext,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    if let (Some(width), Some(height)) = (self.width, self.height) {
      return Size { width, height };
    }

    let Ok(image) = resolve_image(&self.src, context.global) else {
      return Size::zero();
    };

    let size = match &*image {
      ImageSource::Svg(svg) => Size {
        width: svg.size().width(),
        height: svg.size().height(),
      },
      ImageSource::Bitmap(bitmap) => Size {
        width: bitmap.width() as f32,
        height: bitmap.height() as f32,
      },
    };

    measure_image(size, known_dimensions, available_space)
  }

  fn draw_content(&self, context: &RenderContext, canvas: &mut FastBlendImage, layout: Layout) {
    let Ok(image) = resolve_image(&self.src, context.global) else {
      return;
    };

    draw_image(&image, &self.style, context, canvas, layout);
  }

  fn has_draw_content(&self) -> bool {
    true
  }
}

const DATA_URI_PREFIX: &str = "data:";

fn is_data_uri(src: &str) -> bool {
  src.starts_with(DATA_URI_PREFIX)
}

#[cfg(feature = "image_data_uri")]
fn parse_data_uri_image(src: &str) -> ImageResult {
  use crate::resources::image::load_image_source_from_bytes;
  use base64::{Engine as _, engine::general_purpose};

  let comma_pos = src
    .find(',')
    .ok_or(ImageResourceError::InvalidDataUriFormat)?;

  let metadata = &src[DATA_URI_PREFIX.len()..comma_pos];
  let data = &src[comma_pos + 1..];

  if !metadata.contains("base64") {
    return Err(ImageResourceError::InvalidDataUriFormat);
  }

  let image_bytes = general_purpose::STANDARD
    .decode(data)
    .map_err(|_| ImageResourceError::MalformedDataUri)?;

  load_image_source_from_bytes(&image_bytes)
}

fn resolve_image(src: &str, context: &GlobalContext) -> ImageResult {
  if is_data_uri(src) {
    #[cfg(feature = "image_data_uri")]
    return parse_data_uri_image(src);
    #[cfg(not(feature = "image_data_uri"))]
    return Err(ImageResourceError::DataUriParseNotSupported);
  }

  if is_svg(src) {
    #[cfg(feature = "svg")]
    return crate::resources::image::parse_svg(src);
    #[cfg(not(feature = "svg"))]
    return Err(ImageResourceError::SvgParseNotSupported);
  }

  if let Some(img) = context.persistent_image_store.get(src) {
    return Ok(img);
  }

  Err(ImageResourceError::Unknown)
}

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

  if let Some(width) = width
    && height.is_none()
  {
    height = Some(width / aspect_ratio);
  }

  if let Some(height) = height
    && width.is_none()
  {
    width = Some(height * aspect_ratio);
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
