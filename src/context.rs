use std::{num::NonZeroUsize, sync::Mutex};

use lru::LruCache;

use crate::{font::FontStore, node::draw::ImageFetchCache};

pub struct Context {
  pub image_fetch_cache: ImageFetchCache,
  pub font_store: FontStore,
}

impl Default for Context {
  fn default() -> Self {
    Self {
      image_fetch_cache: Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())),
      font_store: FontStore::default(),
    }
  }
}
