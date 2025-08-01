use takumi::{
  Color, ContainerNode, InheritableStyle,
  LengthUnit::{Percentage, Px},
  Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_border_width() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(255, 255, 255).into()),
      border_width: Px(10.0).into(),
      inheritable_style: InheritableStyle {
        border_color: Some(Color::Rgb(255, 0, 0).into()),
        ..Default::default()
      },
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_border_width.png");
}
