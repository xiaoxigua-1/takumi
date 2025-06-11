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

/// A context for managing fonts in the rendering system.
///
/// This struct holds the font system and cache used for text rendering.
pub struct FontContext {
  /// The font system used for text layout and rendering
  pub font_system: Mutex<FontSystem>,
  /// The cache for font glyphs and metrics
  pub font_cache: Mutex<SwashCache>,
}

/// The main context for image rendering.
///
/// This struct holds all the necessary state for rendering images, including
/// font management, image storage, and debug options.
pub struct Context {
  /// Whether to print the debug tree during layout
  pub print_debug_tree: bool,
  /// Whether to draw debug borders around nodes
  pub draw_debug_border: bool,
  /// The font context for text rendering
  pub font_context: FontContext,
  /// The image store for caching and retrieving images
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
///
/// This is used as the default implementation when no custom ImageStore is provided.
/// It always returns None for get operations and does nothing for insert operations.
#[derive(Default)]
pub struct NoopImageStore;

#[async_trait]
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
  async fn fetch_async(&self, _key: &str) -> Arc<ImageState> {
    unimplemented!("NoopImageStore does not support fetching images")
  }
}

/// Loads a WOFF2 font file and adds it to the font context.
///
/// # Arguments
/// * `font_context` - The font context to add the font to
/// * `font_file` - Path to the WOFF2 font file
///
/// # Returns
/// * `Result<(), FontError>` - Ok if the font was loaded successfully, or an error if loading failed
pub fn load_woff2_font_to_context(
  font_context: &FontContext,
  font_file: &Path,
) -> Result<(), FontError> {
  let font = load_woff2_font(font_file)?;
  let mut system = font_context.font_system.lock().unwrap();
  system.db_mut().load_font_data(font);
  Ok(())
}
