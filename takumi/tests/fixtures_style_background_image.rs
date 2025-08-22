use takumi::layout::{
  node::{ContainerNode, NodeKind},
  style::{
    BackgroundImagesValue, BackgroundPositionsValue, BackgroundRepeat, BackgroundRepeats,
    BackgroundRepeatsValue, BackgroundSizesValue, Color, LengthUnit::Percentage, Style,
  },
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

fn create_container_with(
  background_images: BackgroundImagesValue,
  background_size: Option<BackgroundSizesValue>,
  background_position: Option<BackgroundPositionsValue>,
  background_repeat: Option<BackgroundRepeatsValue>,
) -> ContainerNode<NodeKind> {
  ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_image: Some(background_images.try_into().unwrap()),
      background_size: background_size.map(|v| v.try_into().unwrap()),
      background_position: background_position.map(|v| v.try_into().unwrap()),
      background_repeat: background_repeat.map(|v| v.try_into().unwrap()),
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
    "linear-gradient(45deg, #0000ff, #00ff00), radial-gradient(circle, #000000, transparent)"
      .to_string(),
  );

  let container = create_container(background_images);

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_image_linear_radial_mixed.png",
  );
}

#[test]
fn test_background_no_repeat_center_with_size_px() {
  let images = BackgroundImagesValue::Css(
    "linear-gradient(90deg, rgba(255,0,0,1), rgba(0,0,255,1))".to_string(),
  );
  let container = create_container_with(
    images,
    Some(BackgroundSizesValue::Css("200px 120px".to_string())),
    Some(BackgroundPositionsValue::Css("center center".to_string())),
    Some(BackgroundRepeatsValue::Css("no-repeat".to_string())),
  );

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_no_repeat_center_200x120.png",
  );
}

#[test]
fn test_background_repeat_tile_from_top_left() {
  let images = BackgroundImagesValue::Css(
    "linear-gradient(90deg, rgba(0,200,0,1), rgba(0,0,0,0))".to_string(),
  );
  let container = create_container_with(
    images,
    Some(BackgroundSizesValue::Css("160px 100px".to_string())),
    Some(BackgroundPositionsValue::Css("0 0".to_string())),
    Some(BackgroundRepeatsValue::Css("repeat".to_string())),
  );

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_repeat_tile_from_top_left.png",
  );
}

#[test]
fn test_background_repeat_space() {
  let images = BackgroundImagesValue::Css(
    "radial-gradient(circle, rgba(255,165,0,1) 0%, rgba(255,165,0,0) 70%)".to_string(),
  );
  let container = create_container_with(
    images,
    Some(BackgroundSizesValue::Css("120px 120px".to_string())),
    None,
    Some(BackgroundRepeatsValue::Css("space".to_string())),
  );

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_repeat_space.png",
  );
}

#[test]
fn test_background_repeat_round() {
  let images = BackgroundImagesValue::Css(
    "radial-gradient(circle, rgba(0,0,0,1) 0%, rgba(0,0,0,0) 60%)".to_string(),
  );
  let container = create_container_with(
    images,
    Some(BackgroundSizesValue::Css("180px 120px".to_string())),
    None,
    Some(BackgroundRepeatsValue::Css("round".to_string())),
  );

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_repeat_round.png",
  );
}

#[test]
fn test_background_position_percentage_with_no_repeat() {
  let images = BackgroundImagesValue::Css(
    "linear-gradient(0deg, rgba(255,0,255,1), rgba(255,0,255,0))".to_string(),
  );
  let container = create_container_with(
    images,
    Some(BackgroundSizesValue::Css("220px 160px".to_string())),
    Some(BackgroundPositionsValue::Css("25% 75%".to_string())),
    Some(BackgroundRepeatsValue::Css("no-repeat".to_string())),
  );

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_position_percent_25_75.png",
  );
}

#[test]
fn test_background_size_percentage_with_repeat() {
  let images = BackgroundImagesValue::Css(
    "linear-gradient(180deg, rgba(0,128,255,0.9), rgba(0,128,255,0))".to_string(),
  );
  let container = create_container_with(
    images,
    Some(BackgroundSizesValue::Css("20% 20%".to_string())),
    Some(BackgroundPositionsValue::Css("0 0".to_string())),
    Some(BackgroundRepeatsValue::Css("repeat".to_string())),
  );

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_size_percent_20_20.png",
  );
}

#[test]
fn test_background_image_grid_pattern() {
  let images = BackgroundImagesValue::Css(
    "linear-gradient(to right, grey 1px, transparent 1px), linear-gradient(to bottom, grey 1px, transparent 1px)".to_string(),
  );

  let mut container = create_container_with(
    images,
    Some(BackgroundSizesValue::Css("40px 40px".to_string())),
    Some(BackgroundPositionsValue::Css("0 0, 0 0".to_string())),
    Some(BackgroundRepeatsValue::Css("repeat, repeat".to_string())),
  );

  container.style.background_color = Some(Color([255, 255, 255, 255]));

  assert_eq!(
    container.style.background_repeat,
    Some(BackgroundRepeats(vec![
      BackgroundRepeat::repeat(),
      BackgroundRepeat::repeat()
    ]))
  );

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_image_grid_pattern.png",
  );
}
