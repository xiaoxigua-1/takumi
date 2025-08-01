use takumi::style::Display;
use takumi::{
  Color, ContainerNode, FlexDirection, InheritableStyle,
  LengthUnit::{Percentage, Px},
  Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_flex_basis() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      display: Display::Flex,
      flex_direction: FlexDirection::Row,
      background_color: Some(Color::Rgb(0, 0, 255).into()), // Blue background to show container
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          flex_basis: Px(100.0), // Set flex basis to 100px
          height: Px(50.0),
          background_color: Some(Color::Rgb(255, 0, 0).into()), // Red child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
      ContainerNode {
        style: Style {
          flex_basis: Px(100.0), // Set flex basis to 100px
          height: Px(50.0),
          background_color: Some(Color::Rgb(0, 255, 0).into()), // Green child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
      ContainerNode {
        style: Style {
          flex_basis: Px(100.0), // Set flex basis to 100px
          height: Px(50.0),
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

  run_style_width_test(container.into(), "tests/fixtures/style_flex_basis.png");
}
