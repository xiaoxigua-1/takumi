use std::{io::Cursor, sync::Arc};

use base64::{Engine, prelude::BASE64_STANDARD};
use serde_wasm_bindgen::from_value;
use takumi::{
  GlobalContext,
  image::load_from_memory,
  layout::{Viewport, node::NodeKind},
  parley::{FontWeight, fontique::FontInfoOverride},
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
  WebP = "webp",
  Png = "png",
  Jpeg = "jpeg",
}

#[wasm_bindgen]
pub struct FontInfo {
  name: Option<String>,
  data: Vec<u8>,
  weight: Option<f64>,
  style: Option<FontStyle>,
}

#[wasm_bindgen]
pub enum FontStyle {
  Normal = "normal",
  Italic = "italic",
  Oblique = "oblique",
}

impl From<FontStyle> for takumi::parley::FontStyle {
  fn from(style: FontStyle) -> Self {
    match style {
      FontStyle::Italic => takumi::parley::FontStyle::Italic,
      FontStyle::Oblique => takumi::parley::FontStyle::Oblique(None),
      FontStyle::Normal | FontStyle::__Invalid => takumi::parley::FontStyle::Normal,
    }
  }
}

impl From<ImageOutputFormat> for takumi::rendering::ImageOutputFormat {
  fn from(format: ImageOutputFormat) -> Self {
    match format {
      ImageOutputFormat::WebP => takumi::rendering::ImageOutputFormat::WebP,
      ImageOutputFormat::Jpeg => takumi::rendering::ImageOutputFormat::Jpeg,
      ImageOutputFormat::Png | ImageOutputFormat::__Invalid => {
        takumi::rendering::ImageOutputFormat::Png
      }
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

  #[wasm_bindgen(js_name = loadFontWithInfo)]
  pub fn load_font_with_info(&self, font_data: FontInfo) {
    self
      .context
      .font_context
      .load_and_store(
        &font_data.data,
        Some(FontInfoOverride {
          family_name: font_data.name.as_deref(),
          style: font_data.style.map(Into::into),
          weight: font_data
            .weight
            .map(|weight| FontWeight::new(weight as f32)),
          axes: None,
          width: None,
        }),
      )
      .unwrap();
  }

  #[wasm_bindgen(js_name = loadFont)]
  pub fn load_font(&self, buffer: &[u8]) {
    self
      .context
      .font_context
      .load_and_store(buffer, None)
      .unwrap();
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
