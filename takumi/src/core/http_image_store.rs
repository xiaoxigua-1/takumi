use std::{
  num::NonZeroUsize,
  sync::{Arc, Mutex},
};

use minreq::{Method::Get, Request};

use crate::{ImageState, ImageStore, resources::ImageError};

/// A default implementation of `ImageStore` that uses a LRU cache and minreq.
pub struct HttpImageStore(Mutex<lru::LruCache<String, Arc<ImageState>>>);

impl HttpImageStore {
  /// Creates a new `DefaultImageStore` with the given maximum cache size.
  #[must_use]
  pub fn new(max_size: NonZeroUsize) -> Self {
    Self(Mutex::new(lru::LruCache::new(max_size)))
  }
}

impl ImageStore for HttpImageStore {
  fn get(&self, key: &str) -> Option<Arc<ImageState>> {
    self.0.lock().unwrap().get(key).cloned()
  }

  fn insert(&self, key: String, value: Arc<ImageState>) {
    self.0.lock().unwrap().put(key, value);
  }

  fn fetch(&self, key: &str) -> ImageState {
    let body = Request::new(Get, key)
      .send()
      .map_err(|_| ImageError::NetworkError)?;

    let image = image::load_from_memory(body.as_bytes()).map_err(ImageError::DecodeError)?;

    Ok(image.into())
  }

  fn clear(&self) {
    self.0.lock().unwrap().clear();
  }

  fn count(&self) -> usize {
    self.0.lock().unwrap().len()
  }
}
