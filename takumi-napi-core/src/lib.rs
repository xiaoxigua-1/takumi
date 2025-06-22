use std::io::Cursor;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use takumi::{
  context::GlobalContext,
  image::ImageFormat,
  node::{DefaultNodeKind, Node},
  render::{ImageRenderer, Viewport},
};

#[napi]
pub struct Renderer(GlobalContext);

#[napi(object)]
pub struct RenderOptions {
  pub fonts: Vec<Buffer>,
}

#[napi]
impl Renderer {
  #[napi(constructor)]
  pub fn new(options: RenderOptions) -> Self {
    let context = GlobalContext::default();

    let mut system = context.font_context.font_system.lock().unwrap();
    for font in options.fonts {
      system.db_mut().load_font_data(font.into());
    }

    Self(GlobalContext::default())
  }

  #[napi(ts_args_type = "source: { type: string }")]
  pub fn render(&self, env: &Env, source: Object) -> Result<Buffer> {
    let mut node: DefaultNodeKind = env.from_js_value(source)?;

    node.inherit_style_for_children();
    node.hydrate(&self.0);

    let mut render: ImageRenderer<DefaultNodeKind> = ImageRenderer::new(Viewport::new(1200, 630));

    render.construct_taffy_tree(node, &self.0);
    render
      .draw(&self.0)
      .map_err(|err| napi::Error::from_reason(format!("{err:?}")))?;

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    let image = render.draw(&self.0).unwrap();

    image.write_to(&mut cursor, ImageFormat::WebP).unwrap();

    Ok(buffer.into())
  }
}
