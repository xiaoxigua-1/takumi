use std::{
  num::NonZeroUsize,
  sync::{Arc, Mutex},
};

use async_trait::async_trait;
use lru::LruCache;
use reqwest::Client;
use takumi::{
  context::ImageStore as ImageStoreTrait, image::load_from_memory, node::draw::ImageState,
};

#[derive(Debug)]
pub struct ImageStore {
  store: Mutex<LruCache<String, Arc<ImageState>>>,
  http: Client,
}

impl ImageStore {
  pub fn new(http: Client) -> Self {
    Self {
      store: Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())),
      http,
    }
  }
}

#[async_trait]
impl ImageStoreTrait for ImageStore {
  fn get(&self, key: &str) -> Option<Arc<ImageState>> {
    self.store.lock().unwrap().get(key).cloned()
  }

  fn insert(&self, key: String, value: Arc<ImageState>) {
    self.store.lock().unwrap().put(key, value);
  }

  async fn fetch_async(&self, key: &str) -> Arc<ImageState> {
    let Ok(response) = self.http.get(key).send().await else {
      return Arc::new(ImageState::NetworkError);
    };

    let Ok(body) = response.bytes().await else {
      return Arc::new(ImageState::NetworkError);
    };

    let image = load_from_memory(body.as_ref());

    if let Err(e) = image {
      return Arc::new(ImageState::DecodeError(e));
    }

    Arc::new(ImageState::Fetched(image.unwrap().into()))
  }
}
