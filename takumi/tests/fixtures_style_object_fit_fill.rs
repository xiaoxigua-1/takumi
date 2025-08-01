use takumi::{
  Color, ContainerNode, ImageNode, LengthUnit::Percentage, LengthUnit::Px, ObjectFit, Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_object_fit_fill() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color::Rgb(0, 0, 255).into()), // Blue background to serve as container
      ..Default::default()
    },
    children: Some(vec![
      ImageNode {
        style: Style {
          width: Px(100.0),
          height: Px(100.0),
          object_fit: ObjectFit::Fill,
          ..Default::default()
        },
        src: "assets/images/yeecord.png".to_string(),
        image: std::sync::Arc::new(std::sync::OnceLock::new()),
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/style_object_fit_fill.png");
}
