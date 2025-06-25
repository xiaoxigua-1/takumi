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
}

#[wasm_bindgen]
pub fn render(node: JsValue, options: Options) -> Vec<u8> {
  let mut node: DefaultNodeKind = from_value(node).unwrap();

  let context = GlobalContext {
    draw_debug_border: options.debug.unwrap_or_default(),
    ..Default::default()
  };

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
