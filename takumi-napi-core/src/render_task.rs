use std::io::Cursor;

use napi::bindgen_prelude::*;
use takumi::{
  GlobalContext,
  layout::{
    Viewport,
    node::{Node, NodeKind},
  },
  rendering::{ImageOutputFormat, ImageRenderer, write_image},
};

pub struct RenderTask<'ctx> {
  pub node: Option<NodeKind>,
  pub context: &'ctx GlobalContext,
  pub viewport: Viewport,
  pub format: ImageOutputFormat,
  pub quality: Option<u8>,
}

impl<'ctx> Task for RenderTask<'ctx> {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut node = self.node.take().unwrap();

    node.inherit_style_for_children();

    let mut render = ImageRenderer::new(self.viewport);

    render.construct_taffy_tree(node, self.context);

    let image = render
      .draw(self.context)
      .map_err(|e| napi::Error::from_reason(format!("Failed to draw: {e:?}")))?;

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
