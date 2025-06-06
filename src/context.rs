use std::{num::NonZeroUsize, path::Path, sync::Mutex};

use cosmic_text::{FontSystem, SwashCache};
use lru::LruCache;

use crate::{
  font::{FontError, load_woff2_font},
  node::draw::ImageFetchCache,
};

pub struct Context {
  pub image_fetch_cache: ImageFetchCache,
  pub print_debug_tree: bool,
  pub draw_debug_border: bool,
  pub font_system: Mutex<FontSystem>,
  pub font_cache: Mutex<SwashCache>,
}

impl Default for Context {
  fn default() -> Self {
    Self {
      image_fetch_cache: Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())),
      print_debug_tree: false,
      draw_debug_border: false,
      font_system: Mutex::new(FontSystem::new()),
      font_cache: Mutex::new(SwashCache::new()),
    }
  }
}

impl Context {
  pub fn load_woff2_font(&self, font_file: &Path) -> Result<(), FontError> {
    let font = load_woff2_font(font_file)?;
    let mut system = self.font_system.lock().unwrap();
    system.db_mut().load_font_data(font);
    Ok(())
  }
}
