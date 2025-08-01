use takumi::{
  BoxShadow, BoxShadowInput, Color, ContainerNode,
  LengthUnit::{Percentage, Px},
  Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_box_shadow() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(0, 0, 255).into()), // Blue background to serve as container
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Px(100.0),
          height: Px(100.0),
          background_color: Some(Color::Rgb(255, 0, 0).into()), // Red child to show box shadow effect
          box_shadow: Some(BoxShadowInput::Single(BoxShadow {
            color: Color::Rgba(0, 0, 0, 0.5).into(), // Semi-transparent black shadow
            offset_x: Px(5.0),
            offset_y: Px(5.0),
            blur_radius: Px(10.0),
            spread_radius: Px(0.0),
            inset: false,
          })),
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/style_box_shadow.png");
}
