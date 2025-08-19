use napi::Task;
use takumi::{GlobalContext, resources::image::load_image_source_from_bytes};

pub struct PutPersistentImageTask<'ctx> {
  pub src: Option<String>,
  pub context: &'ctx GlobalContext,
  pub buffer: Vec<u8>,
}

impl<'ctx> Task for PutPersistentImageTask<'ctx> {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let image = load_image_source_from_bytes(&self.buffer).unwrap();
    self
      .context
      .persistent_image_store
      .insert(&self.src.take().unwrap(), image);

    Ok(())
  }

  fn resolve(&mut self, _env: napi::Env, _output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(())
  }
}
