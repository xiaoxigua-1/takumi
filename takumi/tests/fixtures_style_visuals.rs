use takumi::layout::{
  node::{ContainerNode, TextNode},
  style::{
    BoxShadow, BoxShadows, Color, InheritableStyle,
    LengthUnit::{Percentage, Px, Rem},
    LineHeight, Position, StyleBuilder,
  },
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_background_color() {
  let container = ContainerNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .background_color(Some(Color([255, 0, 0, 255])))
      .build()
      .unwrap(),
    children: None,
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_color.png",
  );
}

#[test]
fn test_style_border_radius() {
  let container = ContainerNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .background_color(Some(Color([255, 0, 0, 255])))
      .border_radius(Some(Px(20.0).into()))
      .build()
      .unwrap(),
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_border_radius.png");
}

#[test]
fn test_style_border_radius_per_corner() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color([255, 0, 0, 255])),
      // Per-corner radii
      border_top_left_radius: Some(Px(40.0)),
      border_top_right_radius: Some(Px(10.0)),
      border_bottom_right_radius: Some(Px(80.0)),
      border_bottom_left_radius: Some(Px(0.0)),
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_border_radius_per_corner.png",
  );
}

#[test]
fn test_style_border_width() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::white()),
      border_width: Px(10.0).into(),
      inheritable_style: InheritableStyle {
        border_color: Some(Color([255, 0, 0, 255])),
        ..Default::default()
      },
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_border_width.png");
}

#[test]
fn test_style_border_width_with_radius() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      padding: Rem(4.0).into(),
      background_color: Some(Color::white()),
      inheritable_style: InheritableStyle {
        border_color: Some(Color([255, 0, 0, 255])),
        ..Default::default()
      },
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Rem(16.0),
          height: Rem(8.0),
          border_radius: Some(Px(10.0).into()),
          border_width: Px(4.0).into(),
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_border_width_with_radius.png",
  );
}

#[test]
fn test_style_box_shadow() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color([0, 0, 255, 255])), // Blue background to serve as container
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Px(100.0),
          height: Px(100.0),
          background_color: Some(Color([255, 0, 0, 255])), // Red child to show box shadow effect
          box_shadow: Some(BoxShadows(vec![BoxShadow {
            color: Color([0, 0, 0, 128]), // Semi-transparent black shadow
            offset_x: Px(5.0),
            offset_y: Px(5.0),
            blur_radius: Px(10.0),
            spread_radius: Px(0.0),
            inset: false,
          }])),
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/style_box_shadow.png");
}

#[test]
fn test_style_box_shadow_inset() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color([0, 0, 255, 255])), // Blue background container for contrast
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Px(120.0),
          height: Px(80.0),
          background_color: Some(Color::white()), // White child for inset visibility
          border_radius: Some(Px(16.0).into()),
          box_shadow: Some(BoxShadows(vec![BoxShadow {
            color: Color([0, 0, 0, 153]),
            offset_x: Px(4.0),
            offset_y: Px(6.0),
            blur_radius: Px(18.0),
            spread_radius: Px(8.0),
            inset: true,
          }])),
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_box_shadow_inset.png",
  );
}

#[test]
fn test_style_position() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color([0, 0, 255, 255])), // Blue background to serve as container
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Px(100.0),
          height: Px(100.0),
          position: Position::Absolute, // Test the position property
          inset: Px(20.0).into(),       // Position with inset properties
          background_color: Some(Color([255, 0, 0, 255])), // Red child to make it visible
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/style_position.png");
}

#[test]
fn test_style_border_radius_circle() {
  let container = ContainerNode {
    style: Style {
      width: Px(300.0),
      height: Px(300.0),
      background_color: Some(Color([255, 0, 0, 255])),
      border_radius: Some(Percentage(50.0).into()),
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_border_radius_circle.png",
  );
}

// https://github.com/kane50613/takumi/issues/151
#[test]
fn test_style_border_radius_width_offset() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color([128, 128, 128, 255])),
      padding: Rem(2.0).into(),
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Percentage(100.0),
          height: Percentage(100.0),
          border_width: Px(1.0).into(),
          border_radius: Some(Px(24.0).into()),
          inheritable_style: InheritableStyle {
            border_color: Some(Color([0, 0, 0, 255])),
            ..Default::default()
          },
          background_color: Some(Color::white()),
          ..Default::default()
        },
        children: Some(vec![
          TextNode {
            text: "The newest blog post".to_string(),
            style: Style {
              width: Percentage(100.0),
              padding: Rem(4.0).into(),
              inheritable_style: InheritableStyle {
                font_size: Some(Rem(4.0)),
                font_weight: Some(500.0.into()),
                line_height: Some(LineHeight(Rem(4.0 * 1.5))),
                ..Default::default()
              },
              ..Default::default()
            },
          }
          .into(),
        ]),
      }
      .into(),
    ]),
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_border_radius_width_offset.png",
  );
}
