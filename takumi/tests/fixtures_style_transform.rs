use takumi::layout::{
  node::{ContainerNode, TextNode},
  style::{
    Angle, Color, Display, InheritableStyle,
    LengthUnit::{Percentage, Px},
    Sides, Style, Transform, Transforms,
  },
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_transform_translate_and_scale() {
  let mut container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::white()),
      display: Display::Flex,
      inheritable_style: InheritableStyle {
        font_size: Some(Px(24.0)),
        ..Default::default()
      },
      ..Default::default()
    },
    children: None,
  };

  let position = ContainerNode {
    style: Style {
      width: Px(200.0),
      height: Px(100.0),
      background_color: Some(Color([255, 0, 0, 255])),
      ..Default::default()
    },
    children: Some(vec![
      TextNode {
        text: "200px x 100px".to_string(),
        style: Style::default(),
      }
      .into(),
    ]),
  };

  let translated = ContainerNode {
    style: Style {
      width: Px(300.0),
      height: Px(300.0),
      border_width: Sides([Px(1.0); 4]),
      transform: Some(Transforms(vec![Transform::Translate(Px(100.0), Px(50.0))])),
      background_color: Some(Color([0, 128, 255, 255])),
      ..Default::default()
    },
    children: Some(vec![
      TextNode {
        text: "300px x 300px, translate(100px, 50px)".to_string(),
        style: Style::default(),
      }
      .into(),
    ]),
  };

  let scaled = ContainerNode {
    style: Style {
      transform: Some(Transforms(vec![
        Transform::Translate(Px(0.0), Px(200.0)),
        Transform::Scale(2.0, 2.0),
      ])),
      background_color: Some(Color([0, 255, 0, 255])),
      width: Px(100.0),
      height: Px(100.0),
      border_width: Sides([Px(1.0); 4]),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(12.0)),
        ..Default::default()
      },
      ..Default::default()
    },
    children: Some(vec![
      TextNode {
        text: "100px x 100px, translate(0px, 200px), scale(2.0, 2.0)".to_string(),
        style: Style::default(),
      }
      .into(),
    ]),
  };

  let rotated = ContainerNode {
    style: Style {
      transform: Some(Transforms(vec![Transform::Rotate(Angle::new(45.0))])),
      background_color: Some(Color([0, 0, 255, 255])),
      width: Px(200.0),
      height: Px(200.0),
      border_width: Sides([Px(1.0); 4]),
      inheritable_style: InheritableStyle {
        color: Some(Color::white()),
        ..Default::default()
      },
      ..Default::default()
    },
    children: Some(vec![
      TextNode {
        text: "200px x 200px, rotate(45deg)".to_string(),
        style: Style::default(),
      }
      .into(),
    ]),
  };

  container.children = Some(vec![
    position.into(),
    translated.into(),
    scaled.into(),
    rotated.into(),
  ]);

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_transform_translate_and_scale.png",
  );
}
