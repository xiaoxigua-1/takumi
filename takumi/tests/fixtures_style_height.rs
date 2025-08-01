use takumi::{Color, ContainerNode, LengthUnit::Percentage, Style};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_height() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(255, 255, 255).into()),
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(container.into(), "tests/fixtures/style_height.png");
}
