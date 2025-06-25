use std::io::Cursor;

use serde_wasm_bindgen::from_value;
use takumi::{
  DefaultNodeKind, GlobalContext, ImageRenderer, Node, Viewport,
  rendering::{ImageOutputFormat, write_image},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Options {
  pub viewport: Viewport,
  pub debug: Option<bool>,
  pub format: ImageOutputFormat,
  pub quality: Option<u8>,
  fonts: Option<js_sys::Array>,
}

#[wasm_bindgen]
impl Options {
  #[wasm_bindgen(constructor)]
  pub fn new(viewport: Viewport, format: ImageOutputFormat) -> Options {
    Options {
      viewport,
      debug: None,
      format,
      quality: None,
      fonts: None,
    }
  }

  #[wasm_bindgen(getter)]
  pub fn fonts(&self) -> Option<js_sys::Array> {
    self.fonts.clone()
  }

  #[wasm_bindgen(setter)]
  pub fn set_fonts(&mut self, fonts: Option<js_sys::Array>) {
    self.fonts = fonts;
  }
}

#[wasm_bindgen]
pub fn render(node: JsValue, options: Options) -> Vec<u8> {
  let mut node: DefaultNodeKind = from_value(node).unwrap();

  let context = GlobalContext {
    draw_debug_border: options.debug.unwrap_or_default(),
    ..Default::default()
  };

  if let Some(fonts_array) = options.fonts() {
    for font in fonts_array.iter() {
      let font_data = font.dyn_into::<js_sys::Uint8Array>().unwrap().to_vec();

      context.font_context.load_font(font_data).unwrap();
    }
  }

  node.inherit_style_for_children();
  node.hydrate(&context);

  let mut renderer = ImageRenderer::new(options.viewport);

  renderer.construct_taffy_tree(node, &context);
  let image = renderer.draw(&context).unwrap();

  let mut buffer = Vec::new();
  let mut cursor = Cursor::new(&mut buffer);

  write_image(&image, &mut cursor, options.format, options.quality).unwrap();

  buffer
}
