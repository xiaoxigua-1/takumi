use std::num::NonZeroUsize;

use super::{DefaultImageStore, FontContext, ImageStore};

#[cfg(feature = "default_impl")]
use reqwest::blocking;

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
      image_store: Box::new(super::NoopImageStore),
    }
  }
}
