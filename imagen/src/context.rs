use std::{
  path::Path,
  sync::{Arc, Mutex},
};

use async_trait::async_trait;
use cosmic_text::{FontSystem, SwashCache};

use crate::{
  font::{FontError, load_woff2_font},
  node::draw::ImageState,
};

/// A trait for storing and retrieving images in an image rendering system.
///
/// This trait allows implementors to provide their own image storage and caching mechanisms.
/// The trait is designed to be thread-safe and can be used in async contexts.
///
/// # Example
/// ```rust
/// use std::sync::Arc;
/// use imagen::context::ImageStore;
/// use imagen::node::draw::ImageState;
///
/// struct MyImageStore;
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
///     async fn fetch_async(&self, key: &str) -> Arc<ImageState> {
///         // Implement async image fetching
///         unimplemented!()
///     }
/// }
/// ```
#[async_trait]
pub trait ImageStore: Send + Sync {
  /// Retrieves an image from the store by its key.
  ///
  /// # Arguments
  /// * `key` - The unique identifier for the image
  ///
  /// # Returns
  /// * `Option<Arc<ImageState>>` - The image if found, None otherwise
  fn get(&self, key: &str) -> Option<Arc<ImageState>>;

  /// Stores an image in the store with the given key.
  ///
  /// # Arguments
  /// * `key` - The unique identifier for the image
  /// * `value` - The image to store
  fn insert(&self, key: String, value: Arc<ImageState>);

  /// Asynchronously fetches an image from a remote source and stores it.
  ///
  /// # Arguments
  /// * `key` - The unique identifier for the image
  ///
  /// # Returns
  /// * `Arc<ImageState>` - The fetched image
  async fn fetch_async(&self, key: &str) -> Arc<ImageState>;
}

pub struct FontContext {
  pub font_system: Mutex<FontSystem>,
  pub font_cache: Mutex<SwashCache>,
}

pub struct Context {
  pub print_debug_tree: bool,
  pub draw_debug_border: bool,
  pub font_context: FontContext,
  pub image_store: Box<dyn ImageStore>,
}

impl Default for Context {
  fn default() -> Self {
    Self {
      print_debug_tree: false,
      draw_debug_border: false,
      font_context: FontContext {
        font_system: Mutex::new(FontSystem::new()),
        font_cache: Mutex::new(SwashCache::new()),
      },
      image_store: Box::new(NoopImageStore),
    }
  }
}

/// A no-op implementation of ImageStore that does nothing.
/// This is used as the default implementation when no custom ImageStore is provided.
#[derive(Default)]
pub struct NoopImageStore;

#[async_trait]
impl ImageStore for NoopImageStore {
  fn get(&self, _key: &str) -> Option<Arc<ImageState>> {
    None
  }

  fn insert(&self, _key: String, _value: Arc<ImageState>) {
    // No-op
  }

  async fn fetch_async(&self, _key: &str) -> Arc<ImageState> {
    unimplemented!("NoopImageStore does not support fetching images")
  }
}

pub fn load_woff2_font_to_context(
  font_context: &FontContext,
  font_file: &Path,
) -> Result<(), FontError> {
  let font = load_woff2_font(font_file)?;
  let mut system = font_context.font_system.lock().unwrap();
  system.db_mut().load_font_data(font);
  Ok(())
}
