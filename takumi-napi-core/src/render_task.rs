use std::io::Cursor;

use napi::bindgen_prelude::*;
use std::sync::Arc;
use takumi::{
  GlobalContext,
  layout::{Viewport, node::NodeKind},
  rendering::{ImageOutputFormat, render, write_image},
};

pub struct RenderTask {
  pub node: Option<NodeKind>,
  pub context: Arc<GlobalContext>,
  pub viewport: Viewport,
  pub format: ImageOutputFormat,
  pub quality: Option<u8>,
}

impl Task for RenderTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let node = self.node.take().unwrap();

    let image = render(self.viewport, &self.context, node)
      .map_err(|e| napi::Error::from_reason(format!("Failed to render: {e:?}")))?;

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    write_image(&image, &mut cursor, self.format, self.quality)
      .map_err(|e| napi::Error::from_reason(format!("Failed to write to buffer: {e:?}")))?;

    Ok(buffer)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}
