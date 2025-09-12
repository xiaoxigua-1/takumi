use takumi::layout::{
  node::ContainerNode,
  style::{
    AlignItems, Color, Display, JustifyContent,
    LengthUnit::{Percentage, Px},
    StyleBuilder,
  },
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_align_items() {
  let container = ContainerNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .display(Display::Flex)
      .align_items(Some(AlignItems::Center))
      .background_color(Color([0, 0, 255, 255]))
      .build()
      .unwrap(),
    children: Some(vec![
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Color([255, 0, 0, 255]))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Color([0, 255, 0, 255]))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Color([255, 255, 0, 255]))
          .build()
          .unwrap(),
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
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .display(Display::Flex)
      .justify_content(Some(JustifyContent::Center))
      .background_color(Color([0, 0, 255, 255]))
      .build()
      .unwrap(),
    children: Some(vec![
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Color([255, 0, 0, 255]))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Color([0, 255, 0, 255]))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Color([255, 255, 0, 255]))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/style_justify_content.png");
}
