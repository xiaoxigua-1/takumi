use std::sync::Mutex;

use image::{
  ImageError, RgbaImage,
  imageops::{FilterType, overlay, resize},
  load_from_memory,
};
use lru::LruCache;
use serde::Deserialize;
use taffy::{Layout, Style};

use crate::{context::Context, node::Node};

pub type ImageFetchCache = Mutex<LruCache<String, ImageState>>;

#[derive(Debug)]
pub enum ImageState {
  Fetched(RgbaImage),
  NetworkError(reqwest::Error),
  DecodeError(ImageError),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageNode {
  pub src: String,
  pub style: Style,
}

impl ImageNode {
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

    let state = self.fetch_state().await;

    let mut cache = context.image_fetch_cache.lock().unwrap();
    cache.put(self.src.clone(), state);
  }

  async fn fetch_state(&self) -> ImageState {
    let response = reqwest::get(&self.src).await;

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

impl Node for ImageNode {
  fn get_style(&self) -> Style {
    self.style.clone()
  }

  fn render(&self, context: &Context, canvas: &mut RgbaImage, layout: Layout) {
    let mut lock = context.image_fetch_cache.lock().unwrap();

    let Some(ImageState::Fetched(image)) = lock.get(&self.src) else {
      return;
    };

    let should_resize = layout.content_size.width as u32 != image.width()
      || layout.content_size.height as u32 != image.height();

    if !should_resize {
      return overlay(
        canvas,
        image,
        (layout.location.x + layout.padding.left) as i64,
        (layout.location.y + layout.padding.top) as i64,
      );
    }

    let resized = resize(
      image,
      layout.content_size.width as u32,
      layout.content_size.height as u32,
      FilterType::Lanczos3,
    );

    overlay(
      canvas,
      &resized,
      (layout.location.x + layout.padding.left) as i64,
      (layout.location.y + layout.padding.top) as i64,
    );
  }
}
