use std::{
  num::NonZeroUsize,
  sync::{Mutex, RwLock},
};

use lru::LruCache;

use crate::{font::FontStore, node::image_node::ImageFetchCache};

pub struct Context {
  pub image_fetch_cache: ImageFetchCache,
  pub font_store: FontStore,
}

impl Default for Context {
  fn default() -> Self {
    Self {
      image_fetch_cache: Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())),
      font_store: RwLock::default(),
    }
  }
}
