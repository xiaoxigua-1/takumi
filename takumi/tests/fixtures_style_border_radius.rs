use takumi::{
  Color, ContainerNode, InheritableStyle,
  LengthUnit::{Percentage, Px},
  SidesValue, Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_border_radius() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(255, 0, 0).into()), // Red background to show rounded corners
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
