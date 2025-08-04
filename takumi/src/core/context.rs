use crate::core::{FontContext, PersistentImageStore};

/// The main context for image rendering.
///
/// This struct holds all the necessary state for rendering images, including
/// font management, image storage, and debug options.
pub struct GlobalContext {
  /// Whether to draw debug borders around nodes
  pub draw_debug_border: bool,
  /// The font context for text rendering
  pub font_context: FontContext,
  /// The image store for remote contents like http
  pub remote_image_store: Option<Box<dyn crate::ImageStore>>,
  /// The image store for persisting contents
  pub persistent_image_store: PersistentImageStore,
}

impl Default for GlobalContext {
  fn default() -> Self {
    Self {
      draw_debug_border: false,
      font_context: FontContext::default(),
      persistent_image_store: PersistentImageStore::default(),
      #[cfg(feature = "http_image_store")]
      remote_image_store: Some(Box::new(crate::HttpImageStore::new(
        std::num::NonZeroUsize::new(100).unwrap(),
      ))),
      #[cfg(not(feature = "http_image_store"))]
      remote_image_store: None,
    }
  }
}
