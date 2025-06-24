//! Image node implementation for the takumi layout system.
//!
//! This module contains the ImageNode struct which is used to render
//! image content with support for async loading and caching.

use std::sync::{Arc, OnceLock};

use serde::{Deserialize, Serialize};
use taffy::{AvailableSpace, Layout, Size};

use crate::{
  ImageStore,
  core::{GlobalContext, RenderContext},
  layout::{measure_image, trait_node::Node},
  rendering::{FastBlendImage, draw_image},
  resources::ImageState,
  style::Style,
};

/// A node that renders image content.
///
/// Image nodes display images loaded from URLs or file paths,
/// with support for async loading and caching.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageNode {
  /// The styling properties for this image node
  #[serde(default, flatten)]
  pub style: Style,
  /// The source URL or path to the image
  pub src: String,
  /// The cached image state (not serialized)
  #[serde(skip)]
  pub image: Arc<OnceLock<Arc<ImageState>>>,
}

impl<Nodes: Node<Nodes>> Node<Nodes> for ImageNode {
  fn get_style(&self) -> &Style {
    &self.style
  }

  fn get_style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

  fn should_hydrate(&self) -> bool {
    self.image.get().is_none()
  }

  fn hydrate(&self, context: &GlobalContext) {
    if is_data_uri(&self.src) {
      #[cfg(feature = "image_data_uri")]
      {
        let img = parse_data_uri_image(&self.src);
        return self.image.set(Arc::new(img)).unwrap();
      }
      #[cfg(not(feature = "image_data_uri"))]
      {
        return self
          .image
          .set(Arc::new(ImageState::DataUriParseNotSupported))
          .unwrap();
      }
    }

    if let Some(img) = context.local_image_store.get(&self.src) {
      return self.image.set(img).unwrap();
    }

    if let Some(img) = context.image_store.get(&self.src) {
      return self.image.set(img).unwrap();
    }

    let img = context.image_store.fetch(&self.src);

    context.image_store.insert(self.src.clone(), img.clone());
    self.image.set(img).unwrap();
  }

  fn measure(
    &self,
    _context: &RenderContext,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    let Ok(image) = self.image.get().unwrap().as_ref() else {
      return Size::ZERO;
    };

    let (width, height) = image.dimensions();

    measure_image(
      Size {
        width: width as f32,
        height: height as f32,
      },
      known_dimensions,
      available_space,
    )
  }

  fn draw_content(&self, context: &RenderContext, canvas: &mut FastBlendImage, layout: Layout) {
    let Ok(image) = self.image.get().unwrap().as_ref() else {
      return;
    };

    draw_image(image, &self.style, context, canvas, layout);
  }
}

const DATA_URI_PREFIX: &str = "data:";

fn is_data_uri(src: &str) -> bool {
  src.starts_with(DATA_URI_PREFIX)
}

#[cfg(feature = "image_data_uri")]
fn parse_data_uri_image(src: &str) -> ImageState {
  use base64::{Engine as _, engine::general_purpose};

  use crate::resources::ImageError;

  let comma_pos = src.find(',').ok_or(ImageError::InvalidDataUriFormat)?;

  let metadata = &src[DATA_URI_PREFIX.len()..comma_pos];
  let data = &src[comma_pos + 1..];

  if !metadata.contains("base64") {
    return Err(ImageError::InvalidDataUriFormat);
  }

  let image_bytes = general_purpose::STANDARD
    .decode(data)
    .map_err(|_| ImageError::MalformedDataUri)?;

  let img = image::load_from_memory(&image_bytes).map_err(ImageError::DecodeError)?;

  Ok(img.to_rgba8())
}
