use std::{collections::HashMap, sync::RwLock};

use crate::resources::{ImageResult, ImageSource};

/// A trait for storing and retrieving images in an image rendering system.
///
/// This trait allow developers to provide their own image storage and caching mechanisms.
/// The trait is designed to be thread-safe and can be used in async contexts.
///
/// # Example
/// ```rust
/// use std::sync::Arc;
/// use takumi::{ImageStore, ImageResult, ImageSource};
///
/// struct MyImageStore {
///   // http client and image store hashmap
/// }
///
/// impl ImageStore for MyImageStore {
///     fn get(&self, key: &str) -> Option<ImageSource> {
///         // Implement image retrieval
///         None
///     }
///
///     fn insert(&self, key: String, value: ImageSource) {
///         // Implement image storage
///     }
///
///     fn fetch(&self, key: &str) -> ImageResult {
///         // Implement async image fetching
///         unimplemented!()
///     }
///
///     fn clear(&self) {
///         // clear internal storage here
///         unimplemented!()
///     }
///
///     fn count(&self) -> usize {
///         // return items count in your internal storage
///         0
///     }
/// }
/// ```
pub trait ImageStore: Send + Sync {
  /// Retrieves an image from the store by its key.
  fn get(&self, key: &str) -> Option<ImageSource>;

  /// Stores an image in the store with the given key.
  fn insert(&self, key: String, value: ImageSource);

  /// Asynchronously fetches an image from a remote source and stores it.
  fn fetch(&self, key: &str) -> ImageResult;

  /// Clear stored image data
  fn clear(&self);

  /// Retrieves items count of the store
  fn count(&self) -> usize;
}

/// Implementation for storing persistent images, calls to `fetch` function would panic.
#[derive(Default)]
pub struct PersistentImageStore(RwLock<HashMap<String, ImageSource>>);

impl ImageStore for PersistentImageStore {
  fn get(&self, key: &str) -> Option<ImageSource> {
    self.0.read().unwrap().get(key).cloned()
  }

  fn insert(&self, key: String, value: ImageSource) {
    self.0.write().unwrap().insert(key, value);
  }

  fn fetch(&self, _key: &str) -> ImageResult {
    unreachable!()
  }

  fn clear(&self) {
    self.0.write().unwrap().clear();
  }

  fn count(&self) -> usize {
    self.0.read().unwrap().len()
  }
}
