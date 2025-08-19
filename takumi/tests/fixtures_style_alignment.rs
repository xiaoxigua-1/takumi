use takumi::layout::{
  node::ContainerNode,
  style::{
    AlignItems, Color, Display, InheritableStyle, JustifyContent,
    LengthUnit::{Percentage, Px},
    Style,
  },
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
      background_color: Some(Color([0, 0, 255, 255])), // Blue background as container
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Px(50.0),
          height: Px(50.0),
          background_color: Some(Color([255, 0, 0, 255])), // Red child
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
          background_color: Some(Color([0, 255, 0, 255])), // Green child
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
          background_color: Some(Color([255, 255, 0, 255])), // Yellow child
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

#[test]
fn test_style_justify_content() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      display: Display::Flex,
      justify_content: Some(JustifyContent::Center),
      background_color: Some(Color([0, 0, 255, 255])), // Blue background to show container
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Px(50.0),
          height: Px(50.0),
          background_color: Some(Color([255, 0, 0, 255])), // Red child
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
          background_color: Some(Color([0, 255, 0, 255])), // Green child
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
          background_color: Some(Color([255, 255, 0, 255])), // Yellow child
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

  run_style_width_test(container.into(), "tests/fixtures/style_justify_content.png");
}
