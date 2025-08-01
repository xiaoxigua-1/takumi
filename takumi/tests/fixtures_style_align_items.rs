use takumi::{
  AlignItems, Color, ContainerNode, Display, InheritableStyle,
  LengthUnit::{Percentage, Px},
  Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_align_items() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      display: Display::Flex,
      align_items: Some(AlignItems::Center),
      background_color: Some(Color::Rgb(0, 0, 255).into()), // Blue background as container
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Px(50.0),
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
          width: Px(50.0),
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
          width: Px(50.0),
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

  run_style_width_test(container.into(), "tests/fixtures/style_align_items.png");
}
