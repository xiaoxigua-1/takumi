use takumi::{
  BoxShadow, BoxShadows, Color, ContainerNode, InheritableStyle,
  LengthUnit::{Percentage, Px},
  Position, SidesValue, Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_background_color() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color([255, 0, 0, 255])), // Red background
      ..Default::default()
    },
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
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color([255, 0, 0, 255])), // Red background to show rounded corners
      inheritable_style: InheritableStyle {
        border_radius: Some(SidesValue::SingleValue(Px(20.0))), // Uniform rounded corners of 20px
        ..Default::default()
      },
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_border_radius.png");
}

#[test]
fn test_style_border_width() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color([255, 255, 255, 255])),
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
          background_color: Some(Color([255, 255, 255, 255])), // White child for inset visibility
          inheritable_style: InheritableStyle {
            border_radius: Some(SidesValue::SingleValue(Px(16.0))),
            ..Default::default()
          },
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
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/style_position.png");
}
