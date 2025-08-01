use std::{fs::read, path::Path};

use image::{ColorType::Rgba8, load_from_memory, save_buffer};
use imageproc::assert_pixels_eq;
use takumi::{DefaultNodeKind, GlobalContext, ImageRenderer, ImageStore, Node, Viewport};

fn create_test_context() -> GlobalContext {
  let context = GlobalContext::default();

  context.persistent_image_store.insert(
    "assets/images/yeecord.png".into(),
    load_from_memory(include_bytes!("../../assets/images/yeecord.png"))
      .unwrap()
      .into_rgba8()
      .into(),
  );

  context
}

fn create_test_renderer() -> ImageRenderer<DefaultNodeKind> {
  ImageRenderer::new(Viewport::new(1200, 630))
}

/// Helper function to run style width tests
pub fn run_style_width_test(mut node: DefaultNodeKind, fixture_path: &str) {
  let context = create_test_context();
  let mut renderer = create_test_renderer();

  node.inherit_style_for_children();
  node.hydrate(&context).unwrap();

  renderer.construct_taffy_tree(node, &context);

  let image = renderer.draw(&context).unwrap();

  let path = Path::new(fixture_path);

  if cfg!(feature = "test_update_fixtures") {
    save_buffer(path, &image, 1200, 630, Rgba8).expect("Failed to save image");
    return;
  }

  // If fixture doesn't exist, try to create it (for first-time setup)
  // but only if we're not in update mode
  if !path.exists() {
    save_buffer(path, &image, 1200, 630, Rgba8).expect("Failed to save image");
  }

  let fixture = read(path).unwrap();
  let fixture_image = load_from_memory(&fixture).unwrap().into_rgba8();

  assert_pixels_eq!(fixture_image, image);
}
