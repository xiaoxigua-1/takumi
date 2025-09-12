use smallvec::smallvec;
use takumi::layout::{
  node::{ContainerNode, ImageNode, TextNode},
  style::{
    Angle, BackgroundPosition, Color, Display,
    LengthUnit::{Percentage, Px},
    Position, PositionComponent, PositionKeywordX, PositionKeywordY, Sides, Style, StyleBuilder,
    Transform, Transforms,
  },
};

mod test_utils;
use test_utils::run_style_width_test;

const ROTATED_ANGLES: &[f32] = &[0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0];

#[test]
fn test_style_transform_origin_center() {
  let container = ContainerNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .background_color(Color::white())
      .build()
      .unwrap(),
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
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .background_color(Color::white())
      .display(Display::Flex)
      .font_size(Px(24.0))
      .build()
      .unwrap(),
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
    style: StyleBuilder::default()
      .transform(Some(Transforms(smallvec![
        Transform::Translate(Percentage(-50.0), Percentage(-50.0)),
        Transform::Rotate(Angle::new(angle)),
      ])))
      .position(Position::Absolute)
      .inset(Sides([
        Percentage(50.0),
        Percentage(0.0),
        Percentage(0.0),
        Percentage(50.0),
      ]))
      .transform_origin(transform_origin)
      .width(Px(200.0))
      .height(Px(200.0))
      .background_color(Color([255, 0, 0, 30]))
      .border_width(Sides([Px(1.0); 4]))
      .border_radius(Sides([Px(12.0); 4]))
      .build()
      .unwrap(),
    width: None,
    height: None,
    src: "assets/images/yeecord.png".to_string(),
  }
}

#[test]
fn test_style_transform_translate_and_scale() {
  let mut container = ContainerNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .background_color(Color::white())
      .display(Display::Flex)
      .font_size(Px(24.0))
      .build()
      .unwrap(),
    children: None,
  };

  let position = ContainerNode {
    style: StyleBuilder::default()
      .width(Px(200.0))
      .height(Px(100.0))
      .background_color(Color([255, 0, 0, 255]))
      .build()
      .unwrap(),
    children: Some(vec![
      TextNode {
        text: "200px x 100px".to_string(),
        style: Style::default(),
      }
      .into(),
    ]),
  };

  let translated = ContainerNode {
    style: StyleBuilder::default()
      .width(Px(300.0))
      .height(Px(300.0))
      .border_width(Sides([Px(1.0); 4]))
      .transform(Some(Transforms(smallvec![
        Transform::Translate(Px(-100.0), Px(100.0)),
        Transform::Rotate(Angle::new(90.0)),
      ])))
      .background_color(Color([0, 128, 255, 255]))
      .build()
      .unwrap(),
    children: Some(vec![
      ImageNode {
        src: "assets/images/yeecord.png".to_string(),
        style: StyleBuilder::default()
          .width(Percentage(100.0))
          .height(Percentage(100.0))
          .build()
          .unwrap(),
        width: None,
        height: None,
      }
      .into(),
    ]),
  };

  let scaled = ContainerNode {
    style: StyleBuilder::default()
      .transform(Some(Transforms(smallvec![
        Transform::Translate(Px(0.0), Px(200.0)),
        Transform::Scale(2.0, 2.0),
      ])))
      .background_color(Color([0, 255, 0, 255]))
      .width(Px(100.0))
      .height(Px(100.0))
      .border_width(Sides([Px(1.0); 4]))
      .font_size(Px(12.0))
      .build()
      .unwrap(),
    children: Some(vec![
      TextNode {
        text: "100px x 100px, translate(0px, 200px), scale(2.0, 2.0)".to_string(),
        style: Style::default(),
      }
      .into(),
    ]),
  };

  let rotated = ContainerNode {
    style: StyleBuilder::default()
      .transform(Some(Transforms(smallvec![Transform::Rotate(Angle::new(
        45.0
      ))])))
      .background_color(Color([0, 0, 255, 255]))
      .width(Px(200.0))
      .height(Px(200.0))
      .border_width(Sides([Px(1.0); 4]))
      .color(Color::white())
      .build()
      .unwrap(),
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
