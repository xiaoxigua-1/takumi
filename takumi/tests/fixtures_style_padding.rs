use takumi::layout::{
  node::ContainerNode,
  style::{
    Color,
    LengthUnit::{Percentage, Px},
    Sides, StyleBuilder,
  },
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_padding() {
  let container = ContainerNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .background_color(Color([0, 0, 255, 255])) // Blue background to show padding
      .padding(Sides([Px(20.0); 4])) // Uniform padding of 20px
      .build()
      .unwrap(),
    children: Some(vec![
      ContainerNode {
        style: StyleBuilder::default()
          .width(Percentage(100.0))
          .height(Percentage(100.0))
          .background_color(Color([255, 0, 0, 255])) // Red child to show padding effect
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/style_padding.png");
}
