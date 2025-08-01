use std::sync::Arc;

use takumi::{ImageNode, LengthUnit::Px, ObjectFit, Style};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_object_fit_cover() {
  let image = ImageNode {
    style: Style {
      width: Px(100.0),
      height: Px(100.0),
      object_fit: ObjectFit::Cover,
      ..Default::default()
    },
    src: "assets/images/yeecord.png".to_string(),
    image: Arc::default(),
  };

  run_style_width_test(image.into(), "tests/fixtures/style_object_fit_cover.png");
}
