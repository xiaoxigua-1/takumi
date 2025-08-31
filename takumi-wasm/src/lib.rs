use std::{io::Cursor, sync::Arc};

use base64::{Engine, prelude::BASE64_STANDARD};
use serde_wasm_bindgen::from_value;
use takumi::{
  GlobalContext,
  image::load_from_memory,
  layout::{Viewport, node::NodeKind},
  rendering::{render, write_image},
  resources::image::ImageSource,
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

/// Proxy type for the ImageOutputFormat enum.
/// This is needed because wasm-bindgen doesn't support cfg macro in enum variants.
/// https://github.com/erwanvivien/fast_qr/pull/41/files
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageOutputFormat {
  WebP,
  Png,
  Jpeg,
}

impl From<ImageOutputFormat> for takumi::rendering::ImageOutputFormat {
  fn from(format: ImageOutputFormat) -> Self {
    match format {
      ImageOutputFormat::WebP => takumi::rendering::ImageOutputFormat::WebP,
      ImageOutputFormat::Png => takumi::rendering::ImageOutputFormat::Png,
      ImageOutputFormat::Jpeg => takumi::rendering::ImageOutputFormat::Jpeg,
    }
  }
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
  pub fn load_font(&self, font_data: &[u8]) {
    self.context.font_context.load_and_store(font_data).unwrap();
  }

  #[wasm_bindgen(js_name = putPersistentImage)]
  pub fn put_persistent_image(&self, src: String, data: &[u8]) {
    self.context.persistent_image_store.insert(
      &src,
      Arc::new(ImageSource::Bitmap(
        load_from_memory(data).unwrap().into_rgba8(),
      )),
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
  ) -> Vec<u8> {
    let node = node.dyn_into().unwrap();
    let node: NodeKind = from_value(node).unwrap();

    let viewport = Viewport::new(width, height);
    let image = render(viewport, &self.context, node).unwrap();

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    write_image(
      &image,
      &mut cursor,
      format.unwrap_or(ImageOutputFormat::Png).into(),
      quality,
    )
    .unwrap();

    buffer
  }

  #[wasm_bindgen(js_name = "renderAsDataUrl")]
  pub fn render_as_data_url(
    &self,
    node: AnyNode,
    width: u32,
    height: u32,
    format: Option<ImageOutputFormat>,
    quality: Option<u8>,
  ) -> String {
    let buffer = self.render(node, width, height, format, quality);
    let format: takumi::rendering::ImageOutputFormat =
      format.unwrap_or(ImageOutputFormat::Png).into();

    let mut data_uri = String::new();

    data_uri.push_str("data:");
    data_uri.push_str(format.content_type());
    data_uri.push_str(";base64,");
    data_uri.push_str(&BASE64_STANDARD.encode(buffer));

    data_uri
  }
}
