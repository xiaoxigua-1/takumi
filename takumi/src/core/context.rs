use crate::{
  ImageStore,
  core::{FontContext, LocalImageStore},
};

/// The main context for image rendering.
///
/// This struct holds all the necessary state for rendering images, including
/// font management, image storage, and debug options.
pub struct GlobalContext {
  /// Whether to print the debug tree during layout
  pub print_debug_tree: bool,
  /// Whether to draw debug borders around nodes
  pub draw_debug_border: bool,
  /// The font context for text rendering
  pub font_context: FontContext,
  /// The image store for caching and retrieving images
  pub image_store: Box<dyn ImageStore>,
  /// The image store for local contents
  pub local_image_store: LocalImageStore,
}

impl Default for GlobalContext {
  fn default() -> Self {
    Self {
      print_debug_tree: false,
      draw_debug_border: false,
      font_context: FontContext::default(),
      local_image_store: LocalImageStore::default(),
      #[cfg(feature = "image_store_impl")]
      image_store: Box::new(crate::core::DefaultImageStore::new(
        std::num::NonZeroUsize::new(100).unwrap(),
      )),
      #[cfg(not(feature = "image_store_impl"))]
      image_store: Box::new(super::NoopImageStore),
    }
  }
}
