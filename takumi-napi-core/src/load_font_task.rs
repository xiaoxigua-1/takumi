use napi::bindgen_prelude::*;
use rayon::prelude::*;
use takumi::GlobalContext;

pub struct LoadFontTask<'ctx> {
  pub context: &'ctx GlobalContext,
  pub buffers: Vec<Vec<u8>>,
}

impl<'ctx> Task for LoadFontTask<'ctx> {
  type Output = usize;
  type JsValue = u32;

  fn compute(&mut self) -> Result<Self::Output> {
    let buffers = std::mem::take(&mut self.buffers);

    let loaded_count = buffers
      .into_par_iter()
      .map(|buffer| self.context.font_context.load_font(buffer).is_ok() as usize)
      .sum();

    Ok(loaded_count)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output as u32)
  }
}
