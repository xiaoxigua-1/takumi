use std::sync::Arc;

use napi::Task;
use takumi::{
  GlobalContext, ImageStore,
  image::{RgbaImage, load_from_memory},
};

pub struct LoadLocalImageTask<'ctx> {
  pub key: Option<String>,
  pub context: &'ctx GlobalContext,
  pub buffer: Vec<u8>,
}

impl<'ctx> Task for LoadLocalImageTask<'ctx> {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let image: RgbaImage = load_from_memory(&self.buffer).unwrap().into();

    self
      .context
      .local_image_store
      .insert(self.key.take().unwrap(), Arc::new(Ok(image)));

    Ok(())
  }

  fn resolve(&mut self, _env: napi::Env, _output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(())
  }
}
