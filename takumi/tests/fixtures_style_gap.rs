use takumi::{
  Color, ContainerNode, Display, Gap, InheritableStyle,
  LengthUnit::{Percentage, Px},
  Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_gap() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      display: Display::Flex, // Enable flexbox layout to demonstrate gap
      gap: Gap::Array(Px(20.0), Px(20.0)), // Create spacing between children
      background_color: Some(Color::Rgb(0, 0, 255).into()), // Blue background to show container
      ..Default::default()
    },
    children: Some(vec![
      // First child
      ContainerNode {
        style: Style {
          width: Px(50.0),                                      // Fixed width
          height: Px(50.0),                                     // Fixed height
          background_color: Some(Color::Rgb(255, 0, 0).into()), // Red child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
      // Second child
      ContainerNode {
        style: Style {
          width: Px(50.0),                                      // Fixed width
          height: Px(50.0),                                     // Fixed height
          background_color: Some(Color::Rgb(0, 255, 0).into()), // Green child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
      // Third child
      ContainerNode {
        style: Style {
          width: Px(50.0),                                        // Fixed width
          height: Px(50.0),                                       // Fixed height
          background_color: Some(Color::Rgb(255, 255, 0).into()), // Yellow child
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

  run_style_width_test(container.into(), "tests/fixtures/style_gap.png");
}
