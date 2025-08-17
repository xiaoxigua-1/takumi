use std::{num::NonZeroUsize, sync::Mutex};

use minreq::{Method::Get, Request};

use crate::{
  ImageResult, ImageStore,
  resources::{ImageError, ImageSource, load_image_source_from_bytes},
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
    load_image_source_from_bytes(body.as_bytes())
  }

  fn clear(&self) {
    self.0.lock().unwrap().clear();
  }

  fn count(&self) -> usize {
    self.0.lock().unwrap().len()
  }
}
