use image::load_from_memory;
use serde::Deserialize;

use crate::{
  color::Color,
  context::Context,
  node::{Node, draw::ImageState},
};

#[derive(Debug, Clone, Deserialize)]
pub struct RectProperties {
  pub color: Option<Color>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CircleProperties {
  pub color: Option<Color>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextProperties {
  pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageProperties {
  pub src: String,
}

impl ImageProperties {
  pub async fn fetch_and_store(&self, context: &Context) {
    let is_cached = {
      let mut lock = context.image_fetch_cache.lock().unwrap();
      matches!(
        lock.get(&self.src),
        Some(ImageState::Fetched(_) | ImageState::NetworkError(_))
      )
    };

    if is_cached {
      return;
    }

    let state = self.fetch_state(context).await;

    let mut cache = context.image_fetch_cache.lock().unwrap();
    cache.put(self.src.clone(), state);
  }

  async fn fetch_state(&self, context: &Context) -> ImageState {
    let response = context.http.get(&self.src).send().await;

    if let Err(e) = response {
      return ImageState::NetworkError(e);
    }

    let response = response.unwrap();
    let bytes = response.bytes().await;

    if let Err(e) = bytes {
      return ImageState::NetworkError(e);
    }

    let image = load_from_memory(&bytes.unwrap());

    if let Err(e) = image {
      return ImageState::DecodeError(e);
    }

    ImageState::Fetched(image.unwrap().to_rgba8())
  }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ContainerProperties {
  pub children: Vec<Node>,
}
