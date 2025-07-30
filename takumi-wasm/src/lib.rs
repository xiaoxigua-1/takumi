use std::io::Cursor;

use base64::{Engine, prelude::BASE64_STANDARD};
use serde_wasm_bindgen::from_value;
use takumi::{
  DefaultNodeKind, GlobalContext, ImageRenderer, ImageStore, Node, Viewport,
  image::load_from_memory,
  rendering::{ImageOutputFormat, write_image},
  resources::ImageSource,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface AnyNode {
  type: string;
  [key: string]: any;
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "AnyNode")]
  #[derive(Debug)]
  pub type AnyNode;
}

#[wasm_bindgen]
pub struct Renderer {
  context: GlobalContext,
}

#[wasm_bindgen]
impl Renderer {
  #[wasm_bindgen(constructor)]
  pub fn new(debug: Option<bool>) -> Renderer {
    Renderer {
      context: GlobalContext {
        draw_debug_border: debug.unwrap_or_default(),
        ..Default::default()
      },
    }
  }

  #[wasm_bindgen(js_name = loadFont)]
  pub fn load_font(&self, font_data: Vec<u8>) {
    self.context.font_context.load_font(font_data).unwrap();
  }

  #[wasm_bindgen(js_name = putPersistentImage)]
  pub fn put_persistent_image(&self, src: String, data: &[u8]) {
    self.context.persistent_image_store.insert(
      src.to_string(),
      ImageSource::Bitmap(load_from_memory(data).unwrap().into_rgba8()),
    );
  }

  #[wasm_bindgen(js_name = clearImageStore)]
  pub fn clear_image_store(&self) {
    self.context.persistent_image_store.clear();
  }

  #[wasm_bindgen]
  pub fn render(
    &self,
    node: AnyNode,
    width: u32,
    height: u32,
    format: Option<ImageOutputFormat>,
    quality: Option<u8>,
  ) -> Result<Vec<u8>, JsError> {
    let node = node.dyn_into().unwrap();
    let mut node: DefaultNodeKind = from_value(node).unwrap();

    node.inherit_style_for_children();
    node
      .hydrate(&self.context)
      .map_err(|err| JsError::new(&format!("Failed to hydrate node: {err:?}")))?;

    let viewport = Viewport::new(width, height);
    let mut renderer = ImageRenderer::new(viewport);

    renderer.construct_taffy_tree(node, &self.context);
    let image = renderer
      .draw(&self.context)
      .map_err(|err| JsError::new(&format!("Failed to render image: {err}")))?;

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    write_image(
      &image,
      &mut cursor,
      format.unwrap_or(ImageOutputFormat::Png),
      quality,
    )
    .unwrap();

    Ok(buffer)
  }

  #[wasm_bindgen(js_name = "renderAsDataUrl")]
  pub fn render_as_data_url(
    &self,
    node: AnyNode,
    width: u32,
    height: u32,
    format: Option<ImageOutputFormat>,
    quality: Option<u8>,
  ) -> Result<String, JsError> {
    let buffer = self.render(node, width, height, format, quality)?;
    let format = format.unwrap_or(ImageOutputFormat::Png);

    Ok(format!(
      "data:{};base64,{}",
      format.content_type(),
      BASE64_STANDARD.encode(buffer)
    ))
  }
}
