use takumi::{
  Angle, Color, ColorHint, ContainerNode, GradientHint, InheritableStyle,
  LengthUnit::{Percentage, Px},
  LinearGradient, LinearGradients, SidesValue, Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_background_image_gradient_basic() {
  // linear-gradient(45deg, #007aff, #34c759)
  let gradient = LinearGradient {
    steps: vec![
      GradientHint::ColorHint(ColorHint {
        color: Color([0, 122, 255, 255]),
        stop: None,
      }),
      GradientHint::ColorHint(ColorHint {
        color: Color([52, 199, 89, 255]),
        stop: None,
      }),
    ],
    angle: Angle(45.0),
  };

  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      // Gradient used as background_image (per API it supports Gradient)
      background_image: Some(LinearGradients(vec![gradient])),
      // Add radius to exercise rounded background composition path
      inheritable_style: InheritableStyle {
        border_radius: Some(SidesValue::SingleValue(Px(12.0))),
        ..Default::default()
      },
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_image_gradient.png",
  );
}

#[test]
fn test_style_background_image_gradient_alt() {
  // linear-gradient(0deg, #ff3b30, #5856d6)
  let gradient = LinearGradient {
    steps: vec![
      GradientHint::ColorHint(ColorHint {
        color: Color([255, 59, 48, 255]),
        stop: None,
      }),
      GradientHint::ColorHint(ColorHint {
        color: Color([88, 86, 214, 255]),
        stop: Some(1.0),
      }),
    ],
    angle: Angle(0.0),
  };

  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_image: Some(LinearGradients(vec![gradient])),
      ..Default::default()
    },
    children: None,
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_background_image_gradient_alt.png",
  );
}
