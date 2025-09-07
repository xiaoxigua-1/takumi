use napi::bindgen_prelude::*;
use std::sync::Arc;
use takumi::{
  GlobalContext,
  parley::{FontWeight, fontique::FontInfoOverride},
};

use crate::FontInputOwned;

pub struct LoadFontTask {
  pub context: Arc<GlobalContext>,
  pub buffers: Vec<FontInputOwned>,
}

impl Task for LoadFontTask {
  type Output = usize;
  type JsValue = u32;

  fn compute(&mut self) -> Result<Self::Output> {
    if self.buffers.is_empty() {
      return Ok(0);
    }

    let mut loaded_count = 0;

    for buffer in &self.buffers {
      if self
        .context
        .font_context
        .load_and_store(
          &buffer.data,
          Some(FontInfoOverride {
            family_name: buffer.name.as_deref(),
            width: None,
            style: buffer.style.map(Into::into),
            weight: buffer.weight.map(|weight| FontWeight::new(weight as f32)),
            axes: None,
          }),
        )
        .is_ok()
      {
        loaded_count += 1;
      }
    }

    Ok(loaded_count)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output as u32)
  }
}
