use std::sync::Arc;

use napi::bindgen_prelude::*;
use takumi::GlobalContext;

pub struct PreloadImageTask<'ctx> {
  pub context: &'ctx GlobalContext,
  pub url: Option<String>,
}

impl<'ctx> Task for PreloadImageTask<'ctx> {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    let url = self.url.take().unwrap();

    let state = self.context.image_store.fetch(&url);
    let is_fetched = state.is_ok();

    self.context.image_store.insert(url, Arc::new(state));

    Ok(is_fetched)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
