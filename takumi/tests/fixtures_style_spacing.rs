use takumi::{
  Color, ContainerNode, InheritableStyle,
  LengthUnit::{Percentage, Px},
  SidesValue, Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_margin() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(0, 0, 255).into()), // Blue background to show margin
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          margin: SidesValue::SingleValue(Px(20.0)), // Uniform margin of 20px
          width: Px(100.0),                          // Fixed width
          height: Px(100.0),                         // Fixed height
          background_color: Some(Color::Rgb(255, 0, 0).into()), // Red child to show margin effect
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

  run_style_width_test(container.into(), "tests/fixtures/style_margin.png");
}

#[test]
fn test_style_padding() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(0, 0, 255).into()), // Blue background to show padding
      padding: SidesValue::SingleValue(Px(20.0)),           // Uniform padding of 20px
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Percentage(100.0),
          height: Percentage(100.0),
          background_color: Some(Color::Rgb(255, 0, 0).into()), // Red child to show padding effect
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

  run_style_width_test(container.into(), "tests/fixtures/style_padding.png");
}