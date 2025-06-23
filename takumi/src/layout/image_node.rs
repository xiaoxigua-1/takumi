//! Image node implementation for the takumi layout system.
//!
//! This module contains the ImageNode struct which is used to render
//! image content with support for async loading and caching.

use std::sync::{Arc, OnceLock};

use serde::{Deserialize, Serialize};
use taffy::{AvailableSpace, Layout, Size};

use crate::{
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
    let image_store = &context.image_store;

    if let Some(img) = image_store.get(&self.src) {
      self.image.set(img).unwrap();
      return;
    }

    let img = image_store.fetch(&self.src);

    image_store.insert(self.src.clone(), img.clone());
    self.image.set(img).unwrap();
  }

  fn measure(
    &self,
    _context: &RenderContext,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    let ImageState::Fetched(image) = self.image.get().unwrap().as_ref() else {
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
    let ImageState::Fetched(image) = self.image.get().unwrap().as_ref() else {
      return;
    };

    draw_image(image, &self.style, context, canvas, layout);
  }
}
