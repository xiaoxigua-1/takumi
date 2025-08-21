use takumi::layout::{
  node::{ContainerNode, NodeKind},
  style::{BackgroundImagesValue, LengthUnit::Percentage, Style},
};

mod test_utils;
use test_utils::run_style_width_test;

fn create_container(background_images: BackgroundImagesValue) -> ContainerNode<NodeKind> {
  ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_image: Some(background_images.try_into().unwrap()),
      ..Default::default()
    },
    children: None,
  }
}

#[test]
fn test_style_background_image_gradient_basic() {
  let background_images =
    BackgroundImagesValue::Css("linear-gradient(45deg, #007aff, #34c759)".to_string());

  let container = create_container(background_images);

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_image_gradient.png",
  );
}

#[test]
fn test_style_background_image_gradient_alt() {
  // linear-gradient(0deg, #ff3b30, #5856d6)
  let background_images =
    BackgroundImagesValue::Css("linear-gradient(0deg, #ff3b30, #5856d6)".to_string());

  let container = create_container(background_images);

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_image_gradient_alt.png",
  );
}

#[test]
fn test_style_background_image_radial_basic() {
  let background_images =
    BackgroundImagesValue::Css("radial-gradient(#e66465, #9198e5)".to_string());

  let container = create_container(background_images);

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_image_radial_basic.png",
  );
}

#[test]
fn test_style_background_image_radial_mixed() {
  let background_images = BackgroundImagesValue::Css("radial-gradient(ellipse at top, #e66465, transparent), radial-gradient(ellipse at bottom, #4d9f0c, transparent)".to_string());

  let container = create_container(background_images);

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_image_radial_mixed.png",
  );
}

#[test]
fn test_style_background_image_linear_radial_mixed() {
  let background_images = BackgroundImagesValue::Css(
    "radial-gradient(ellipse at top, #e66465, transparent), radial-gradient(ellipse at bottom, #4d9f0c, transparent)".to_string(),
  );

  let container = create_container(background_images);

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_image_linear_radial_mixed.png",
  );
}
