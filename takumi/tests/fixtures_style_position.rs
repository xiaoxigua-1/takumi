use takumi::layout::{
  node::ContainerNode,
  style::{
    Color,
    LengthUnit::{Percentage, Px},
    Position, Style,
  },
};

mod test_utils;
use test_utils::run_style_width_test;

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
