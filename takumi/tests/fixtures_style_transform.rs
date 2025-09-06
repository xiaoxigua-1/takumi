use takumi::layout::{
  node::{ContainerNode, ImageNode, TextNode},
  style::{
    Angle, BackgroundPosition, Color, Display, InheritableStyle,
    LengthUnit::{Percentage, Px},
    Position, PositionComponent, PositionKeywordX, PositionKeywordY, Sides, Style, Transform,
    Transforms,
  },
};

mod test_utils;
use test_utils::run_style_width_test;

const ROTATED_ANGLES: &[f32] = &[0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0];

#[test]
fn test_style_transform_origin_center() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::white()),
      ..Default::default()
    },
    children: Some(
      ROTATED_ANGLES
        .iter()
        .map(|angle| create_rotated_container(*angle, None).into())
        .collect(),
    ),
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_transform_origin_center.png",
  );
}

#[test]
fn test_style_transform_origin_top_left() {
  let container = ContainerNode {
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
    children: Some(
      ROTATED_ANGLES
        .iter()
        .map(|angle| {
          create_rotated_container(
            *angle,
            Some(BackgroundPosition {
              x: PositionComponent::KeywordX(PositionKeywordX::Left),
              y: PositionComponent::KeywordY(PositionKeywordY::Top),
            }),
          )
          .into()
        })
        .collect(),
    ),
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_transform_origin_top_left.png",
  );
}

fn create_rotated_container(angle: f32, transform_origin: Option<BackgroundPosition>) -> ImageNode {
  ImageNode {
    style: Style {
      transform: Some(Transforms(vec![
        Transform::Translate(Percentage(-50.0), Percentage(-50.0)),
        Transform::Rotate(Angle::new(angle)),
      ])),
      position: Position::Absolute,
      inset: Sides([
        Percentage(50.0),
        Percentage(0.0),
        Percentage(0.0),
        Percentage(50.0),
      ]),
      transform_origin,
      width: Px(200.0),
      height: Px(200.0),
      background_color: Some(Color([255, 0, 0, 30])),
      border_width: Sides([Px(1.0); 4]),
      border_radius: Some(Sides([Px(12.0); 4])),
      ..Default::default()
    },
    width: None,
    height: None,
    src: "assets/images/yeecord.png".to_string(),
  }
}

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
      transform: Some(Transforms(vec![
        Transform::Translate(Px(-100.0), Px(100.0)),
        Transform::Rotate(Angle::new(90.0)),
      ])),
      background_color: Some(Color([0, 128, 255, 255])),
      ..Default::default()
    },
    children: Some(vec![
      ImageNode {
        src: "assets/images/yeecord.png".to_string(),
        style: Style {
          width: Percentage(100.0),
          height: Percentage(100.0),
          ..Default::default()
        },
        width: None,
        height: None,
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
