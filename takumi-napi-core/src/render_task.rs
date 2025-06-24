use std::io::Cursor;

use napi::bindgen_prelude::*;
use takumi::{DefaultNodeKind, GlobalContext, ImageRenderer, Node, Viewport, image::ImageFormat};

pub struct RenderTask<'ctx> {
  pub node: Option<DefaultNodeKind>,
  pub context: &'ctx GlobalContext,
  pub viewport: Viewport,
}

impl<'ctx> Task for RenderTask<'ctx> {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut node = self.node.take().unwrap();

    node.inherit_style_for_children();
    node.hydrate(self.context);

    let mut render = ImageRenderer::new(self.viewport);

    render.construct_taffy_tree(node, self.context);
    render
      .draw(self.context)
      .map_err(|err| napi::Error::from_reason(format!("{err:?}")))?;

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    let image = render.draw(self.context).unwrap();

    image.write_to(&mut cursor, ImageFormat::WebP).unwrap();

    Ok(buffer)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}
