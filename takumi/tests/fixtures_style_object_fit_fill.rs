use std::sync::Arc;

use takumi::{ImageNode, LengthUnit::Percentage, ObjectFit, Style};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_object_fit_fill() {
  let image = ImageNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      object_fit: ObjectFit::Fill,
      ..Default::default()
    },
    src: "assets/images/yeecord.png".to_string(),
    image: Arc::default(),
  };

  run_style_width_test(image.into(), "tests/fixtures/style_object_fit_fill.png");
}
