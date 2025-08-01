use takumi::{Color, ContainerNode, LengthUnit::Percentage, Style};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_width() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(255, 255, 255).into()),
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_width.png");
}

#[test]
fn test_style_height() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(255, 255, 255).into()),
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_height.png");
}

#[test]
fn test_style_min_width() {
  let container = ContainerNode {
    style: Style {
      min_width: takumi::LengthUnit::Px(50.0),
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(255, 255, 255).into()),
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_min_width.png");
}

#[test]
fn test_style_min_height() {
  let container = ContainerNode {
    style: Style {
      min_height: takumi::LengthUnit::Px(50.0),
      height: Percentage(100.0),
      width: Percentage(100.0),
      background_color: Some(Color::Rgb(255, 255, 255).into()),
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_min_height.png");
}

#[test]
fn test_style_max_width() {
  let container = ContainerNode {
    style: Style {
      max_width: takumi::LengthUnit::Px(100.0),
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(255, 255, 255).into()),
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_max_width.png");
}

#[test]
fn test_style_max_height() {
  let container = ContainerNode {
    style: Style {
      max_height: takumi::LengthUnit::Px(100.0),
      height: Percentage(100.0),
      width: Percentage(100.0),
      background_color: Some(Color::Rgb(255, 255, 255).into()),
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_max_height.png");
}
