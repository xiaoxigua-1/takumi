//! Image node implementation for the takumi layout system.
//!
//! This module contains the ImageNode struct which is used to render
//! image content with support for async loading and caching.

use std::sync::{Arc, OnceLock};

use serde::Deserialize;
use taffy::{AvailableSpace, Layout, Size};

use crate::{
  ImageStore,
  core::{GlobalContext, RenderContext},
  layout::{measure_image, trait_node::Node},
  rendering::{FastBlendImage, draw_image},
  resources::{ImageError, ImageResult, ImageSource},
  style::Style,
};

/// A node that renders image content.
///
/// Image nodes display images loaded from URLs or file paths,
/// with support for async loading and caching.
#[derive(Debug, Clone, Deserialize)]
pub struct ImageNode {
  /// The styling properties for this image node
  #[serde(default, flatten)]
  pub style: Style,
  /// The source URL or path to the image
  pub src: String,
  /// The cached image state (not serialized)
  #[serde(skip)]
  pub image: Arc<OnceLock<ImageSource>>,
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

  fn hydrate(&self, context: &GlobalContext) -> Result<(), crate::Error> {
    let image = resolve_image(&self.src, context)?;

    self.image.set(image).unwrap();

    Ok(())
  }

  fn measure(
    &self,
    _context: &RenderContext,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    let Some(image) = self.image.get() else {
      return Size::ZERO;
    };

    let (width, height) = image.size();

    measure_image(Size { width, height }, known_dimensions, available_space)
  }

  fn draw_content(&self, context: &RenderContext, canvas: &mut FastBlendImage, layout: Layout) {
    let Some(image) = self.image.get() else {
      return;
    };

    draw_image(image, &self.style, context, canvas, layout);
  }
}

const DATA_URI_PREFIX: &str = "data:";

fn is_data_uri(src: &str) -> bool {
  src.starts_with(DATA_URI_PREFIX)
}

fn is_svg(src: &str) -> bool {
  src.starts_with("<svg") && src.contains("xmlns=\"http://www.w3.org/2000/svg\"")
}

#[cfg(feature = "image_data_uri")]
fn parse_data_uri_image(src: &str) -> ImageResult {
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

  Ok(img.into_rgba8().into())
}

fn resolve_image(src: &str, context: &GlobalContext) -> ImageResult {
  if is_data_uri(src) {
    #[cfg(feature = "image_data_uri")]
    {
      return parse_data_uri_image(src);
    }
    #[cfg(not(feature = "image_data_uri"))]
    {
      return Err(ImageError::DataUriParseNotSupported);
    }
  }

  if is_svg(src) {
    #[cfg(feature = "svg")]
    {
      use resvg::usvg::{Options, Tree};

      let svg_tree = Tree::from_str(src, &Options::default()).map_err(ImageError::SvgParseError)?;

      return Ok(ImageSource::Svg(Box::new(svg_tree)));
    }
    #[cfg(not(feature = "svg"))]
    {
      return Err(ImageError::SvgParseNotSupported);
    }
  }

  if let Some(img) = context.persistent_image_store.get(src) {
    return Ok(img);
  }

  let Some(remote_store) = context.remote_image_store.as_ref() else {
    return Err(ImageError::RemoteStoreNotAvailable);
  };

  if let Some(img) = remote_store.get(src) {
    return Ok(img);
  }

  let img = remote_store.fetch(src)?;

  remote_store.insert(src.to_string(), img.clone());
  Ok(img)
}
