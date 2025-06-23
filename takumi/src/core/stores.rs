use std::{
  num::NonZeroUsize,
  sync::{Arc, Mutex},
};

use crate::resources::ImageState;

/// A trait for storing and retrieving images in an image rendering system.
///
/// This trait allow developers to provide their own image storage and caching mechanisms.
/// The trait is designed to be thread-safe and can be used in async contexts.
///
/// # Example
/// ```rust
/// use std::sync::Arc;
/// use takumi::{ImageStore, ImageState};
///
/// #[derive(Debug)]
/// struct MyImageStore {
///   // http client and image store hashmap
/// }
///
/// impl ImageStore for MyImageStore {
///     fn get(&self, key: &str) -> Option<Arc<ImageState>> {
///         // Implement image retrieval
///         None
///     }
///
///     fn insert(&self, key: String, value: Arc<ImageState>) {
///         // Implement image storage
///     }
///
///     fn fetch(&self, key: &str) -> Arc<ImageState> {
///         // Implement async image fetching
///         unimplemented!()
///     }
///
///     fn clear(&self) {
///         // clear internal storage here
///         unimplemented!()
///     }
/// }
/// ```
pub trait ImageStore: Send + Sync + std::fmt::Debug {
  /// Retrieves an image from the store by its key.
  fn get(&self, key: &str) -> Option<Arc<ImageState>>;

  /// Stores an image in the store with the given key.
  fn insert(&self, key: String, value: Arc<ImageState>);

  /// Asynchronously fetches an image from a remote source and stores it.
  fn fetch(&self, key: &str) -> Arc<ImageState>;

  /// Clear stored image data
  fn clear(&self);
}

#[cfg(feature = "default_impl")]
use reqwest::blocking;

#[cfg(feature = "default_impl")]
/// A default implementation of `ImageStore` that uses a LRU cache and a HTTP client.
#[derive(Debug)]
pub struct DefaultImageStore {
  store: Mutex<lru::LruCache<String, Arc<ImageState>>>,
  http: blocking::Client,
}

#[cfg(feature = "default_impl")]
impl DefaultImageStore {
  /// Creates a new `DefaultImageStore` with the given HTTP client and maximum size.
  #[must_use]
  pub fn new(http: blocking::Client, max_size: NonZeroUsize) -> Self {
    Self {
      store: Mutex::new(lru::LruCache::new(max_size)),
      http,
    }
  }
}

#[cfg(feature = "default_impl")]
impl ImageStore for DefaultImageStore {
  fn get(&self, key: &str) -> Option<Arc<ImageState>> {
    self.store.lock().unwrap().get(key).cloned()
  }

  fn insert(&self, key: String, value: Arc<ImageState>) {
    self.store.lock().unwrap().put(key, value);
  }

  fn fetch(&self, key: &str) -> Arc<ImageState> {
    let Ok(response) = self.http.get(key).send() else {
      return Arc::new(ImageState::NetworkError);
    };

    let Ok(body) = response.bytes() else {
      return Arc::new(ImageState::NetworkError);
    };

    let image = image::load_from_memory(body.as_ref());

    if let Err(e) = image {
      return Arc::new(ImageState::DecodeError(e));
    }

    Arc::new(ImageState::Fetched(image.unwrap().into()))
  }

  fn clear(&self) {
    self.store.lock().unwrap().clear();
  }
}

/// A no-op implementation of `ImageStore` that does nothing.
///
/// This is used as the default implementation when no custom `ImageStore` is provided.
/// It always returns None for get operations and does nothing for insert operations.
#[derive(Default, Debug)]
pub struct NoopImageStore;

impl ImageStore for NoopImageStore {
  /// Always returns None as this is a no-op implementation.
  fn get(&self, _key: &str) -> Option<Arc<ImageState>> {
    None
  }

  /// Does nothing as this is a no-op implementation.
  fn insert(&self, _key: String, _value: Arc<ImageState>) {
    // No-op
  }

  /// Always panics as this is a no-op implementation.
  fn fetch(&self, _key: &str) -> Arc<ImageState> {
    unimplemented!("NoopImageStore does not support fetching images")
  }

  fn clear(&self) {
    // No-op
  }
}
