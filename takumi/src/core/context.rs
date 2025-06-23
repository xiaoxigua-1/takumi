use std::{
  num::NonZeroUsize,
  sync::{Arc, Mutex},
};

use cosmic_text::{FontSystem, SwashCache, fontdb::Database};

use crate::resources::{FontError, ImageState, load_font};

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

/// A context for managing fonts in the rendering system.
///
/// This struct holds the font system and cache used for text rendering.
#[derive(Debug)]
pub struct FontContext {
  /// The font system used for text layout and rendering
  pub font_system: Mutex<FontSystem>,
  /// The cache for font glyphs and metrics
  pub font_cache: Mutex<SwashCache>,
}

impl Default for FontContext {
  fn default() -> Self {
    Self {
      font_system: Mutex::new(FontSystem::new_with_locale_and_db(
        "en-US".to_string(),
        Database::new(),
      )),
      font_cache: Mutex::new(SwashCache::new()),
    }
  }
}

impl FontContext {
  /// Loads font into internal font db
  pub fn load_font(&self, source: Vec<u8>) -> Result<(), FontError> {
    let font_data = load_font(source, None)?;

    let mut lock = self.font_system.lock().unwrap();
    lock.db_mut().load_font_data(font_data);

    Ok(())
  }
}

/// The main context for image rendering.
///
/// This struct holds all the necessary state for rendering images, including
/// font management, image storage, and debug options.
#[derive(Debug)]
pub struct GlobalContext {
  /// Whether to print the debug tree during layout
  pub print_debug_tree: bool,
  /// Whether to draw debug borders around nodes
  pub draw_debug_border: bool,
  /// The font context for text rendering
  pub font_context: FontContext,
  /// The image store for caching and retrieving images
  pub image_store: Box<dyn ImageStore>,
}

#[cfg(feature = "default_impl")]
use reqwest::blocking;

impl Default for GlobalContext {
  fn default() -> Self {
    Self {
      print_debug_tree: false,
      draw_debug_border: false,
      font_context: FontContext::default(),
      #[cfg(feature = "default_impl")]
      image_store: Box::new(DefaultImageStore::new(
        blocking::Client::new(),
        NonZeroUsize::new(100).unwrap(),
      )),
      #[cfg(not(feature = "default_impl"))]
      image_store: Box::new(NoopImageStore),
    }
  }
}

#[cfg(feature = "default_impl")]
/// A default implementation of ImageStore that uses a LRU cache and a HTTP client.
#[derive(Debug)]
pub struct DefaultImageStore {
  store: Mutex<lru::LruCache<String, Arc<ImageState>>>,
  http: blocking::Client,
}

#[cfg(feature = "default_impl")]
impl DefaultImageStore {
  /// Creates a new DefaultImageStore with the given HTTP client and maximum size.
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

/// A no-op implementation of ImageStore that does nothing.
///
/// This is used as the default implementation when no custom ImageStore is provided.
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
