use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
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
///     fn fetch(&self, key: &str) -> ImageState {
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
pub trait ImageStore: Send + Sync {
  /// Retrieves an image from the store by its key.
  fn get(&self, key: &str) -> Option<Arc<ImageState>>;

  /// Stores an image in the store with the given key.
  fn insert(&self, key: String, value: Arc<ImageState>);

  /// Asynchronously fetches an image from a remote source and stores it.
  fn fetch(&self, key: &str) -> ImageState;

  /// Clear stored image data
  fn clear(&self);
}

/// Implementation for storing local images, calls to `fetch` function would panic.
#[derive(Default)]
pub struct LocalImageStore(RwLock<HashMap<String, Arc<ImageState>>>);

impl ImageStore for LocalImageStore {
  fn get(&self, key: &str) -> Option<Arc<ImageState>> {
    self.0.read().unwrap().get(key).cloned()
  }

  fn insert(&self, key: String, value: Arc<ImageState>) {
    self.0.write().unwrap().insert(key, value);
  }

  fn fetch(&self, _key: &str) -> ImageState {
    unreachable!()
  }

  fn clear(&self) {
    self.0.write().unwrap().clear();
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
  fn fetch(&self, _key: &str) -> ImageState {
    unreachable!()
  }

  fn clear(&self) {
    // No-op
  }
}
