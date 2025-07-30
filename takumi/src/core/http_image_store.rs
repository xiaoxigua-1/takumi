use std::{num::NonZeroUsize, sync::Mutex};

use minreq::{Method::Get, Request};

use crate::{
  ImageResult, ImageStore,
  resources::{ImageError, ImageSource},
};

/// A default implementation of `ImageStore` that uses a LRU cache and minreq.
pub struct HttpImageStore(Mutex<lru::LruCache<String, ImageSource>>);

impl HttpImageStore {
  /// Creates a new `DefaultImageStore` with the given maximum cache size.
  #[must_use]
  pub fn new(max_size: NonZeroUsize) -> Self {
    Self(Mutex::new(lru::LruCache::new(max_size)))
  }
}

impl ImageStore for HttpImageStore {
  fn get(&self, key: &str) -> Option<ImageSource> {
    self.0.lock().unwrap().get(key).cloned()
  }

  fn insert(&self, key: String, value: ImageSource) {
    self.0.lock().unwrap().put(key, value);
  }

  fn fetch(&self, key: &str) -> ImageResult {
    let body = Request::new(Get, key)
      .send()
      .map_err(|_| ImageError::NetworkError)?;

    let image = image::load_from_memory(body.as_bytes()).map_err(ImageError::DecodeError)?;

    Ok(image.into_rgba8().into())
  }

  fn clear(&self) {
    self.0.lock().unwrap().clear();
  }

  fn count(&self) -> usize {
    self.0.lock().unwrap().len()
  }
}
