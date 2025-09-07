use std::{path::Path, sync::Arc};

use image::{ColorType::Rgba8, load_from_memory, save_buffer};
use takumi::{
  GlobalContext,
  layout::{Viewport, node::NodeKind},
  rendering::render,
  resources::image::ImageSource,
};

fn create_test_context() -> GlobalContext {
  let context = GlobalContext::default();

  context.persistent_image_store.insert(
    "assets/images/yeecord.png",
    Arc::new(ImageSource::Bitmap(
      load_from_memory(include_bytes!("../../assets/images/yeecord.png"))
        .unwrap()
        .into_rgba8(),
    )),
  );

  context
    .font_context
    .load_and_store(
      include_bytes!("../../assets/fonts/noto-sans/NotoColorEmoji.ttf"),
      None,
    )
    .unwrap();

  context
}

fn create_test_viewport() -> Viewport {
  Viewport::new(1200, 630)
}

/// Helper function to run style width tests
pub fn run_style_width_test(node: NodeKind, fixture_path: &str) {
  let context = create_test_context();
  let viewport = create_test_viewport();

  let image = render(viewport, &context, node).unwrap();

  let path = Path::new(fixture_path);

  save_buffer(path, &image, 1200, 630, Rgba8).expect("Failed to save image");
}
