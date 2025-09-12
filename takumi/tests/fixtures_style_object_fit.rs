use takumi::layout::{
  node::ImageNode,
  style::{LengthUnit::Percentage, ObjectFit, Style, StyleBuilder},
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_object_fit_contain() {
  let image = ImageNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .object_fit(ObjectFit::Contain)
      .build()
      .unwrap(),
    width: None,
    height: None,
    src: "assets/images/yeecord.png".to_string(),
  };

  run_style_width_test(image.into(), "tests/fixtures/style_object_fit_contain.png");
}

#[test]
fn test_style_object_fit_cover() {
  let image = ImageNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .object_fit(ObjectFit::Cover)
      .build()
      .unwrap(),
    width: None,
    height: None,
    src: "assets/images/yeecord.png".to_string(),
  };

  run_style_width_test(image.into(), "tests/fixtures/style_object_fit_cover.png");
}

#[test]
fn test_style_object_fit_fill() {
  let image = ImageNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .object_fit(ObjectFit::Fill)
      .build()
      .unwrap(),
    src: "assets/images/yeecord.png".to_string(),
    width: None,
    height: None,
  };

  run_style_width_test(image.into(), "tests/fixtures/style_object_fit_fill.png");
}

#[test]
fn test_style_object_fit_none() {
  let image = ImageNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .object_fit(ObjectFit::None)
      .build()
      .unwrap(),
    src: "assets/images/yeecord.png".to_string(),
    width: None,
    height: None,
  };

  run_style_width_test(image.into(), "tests/fixtures/style_object_fit_none.png");
}

#[test]
fn test_style_object_fit_scale_down() {
  let image = ImageNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .object_fit(ObjectFit::ScaleDown)
      .build()
      .unwrap(),
    src: "assets/images/yeecord.png".to_string(),
    width: None,
    height: None,
  };

  run_style_width_test(
    image.into(),
    "tests/fixtures/style_object_fit_scale_down.png",
  );
}
