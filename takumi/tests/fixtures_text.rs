use takumi::layout::{
  node::{ContainerNode, NodeKind, TextNode},
  style::{
    BackgroundImagesValue, BackgroundPositionsValue, BackgroundRepeatsValue, BackgroundSizesValue,
    Color, FontWeight, InheritableStyle,
    LengthUnit::{Percentage, Px},
    LineHeight, Style, TextAlign, TextOverflow, TextTransform,
  },
};

mod test_utils;
use test_utils::run_style_width_test;

// Basic text render with defaults
#[test]
fn fixtures_text_basic() {
  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      ..Default::default()
    },
    text: "The quick brown fox jumps over the lazy dog 12345".to_string(),
  };

  run_style_width_test(NodeKind::Text(text), "tests/fixtures/text_basic.png");
}

#[test]
fn fixtures_text_typography_regular_24px() {
  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(24.0)),
        ..Default::default()
      },
      ..Default::default()
    },
    text: "Regular 24px".to_string(),
  };

  run_style_width_test(
    text.into(),
    "tests/fixtures/text_typography_regular_24px.png",
  );
}

#[test]
fn fixtures_text_typography_medium_weight_500() {
  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(24.0)),
        font_weight: Some(FontWeight(500)),
        ..Default::default()
      },
      ..Default::default()
    },
    text: "Medium 24px".to_string(),
  };

  run_style_width_test(
    text.into(),
    "tests/fixtures/text_typography_medium_weight_500.png",
  );
}

#[test]
fn fixtures_text_typography_line_height_40px() {
  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(24.0)),
        line_height: Some(LineHeight(Px(40.0))),
        ..Default::default()
      },
      ..Default::default()
    },
    text: "Line height 40px".to_string(),
  };

  run_style_width_test(
    text.into(),
    "tests/fixtures/text_typography_line_height_40px.png",
  );
}

#[test]
fn fixtures_text_typography_letter_spacing_2px() {
  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(24.0)),
        letter_spacing: Some(Px(2.0)),
        ..Default::default()
      },
      ..Default::default()
    },
    text: "Letter spacing 2px".to_string(),
  };

  run_style_width_test(
    text.into(),
    "tests/fixtures/text_typography_letter_spacing_2px.png",
  );
}

#[test]
fn fixtures_text_align_start() {
  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      width: Percentage(100.0),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(24.0)),
        text_align: Some(TextAlign::Start),
        ..Default::default()
      },
      ..Default::default()
    },
    text: "Start aligned".to_string(),
  };

  run_style_width_test(text.into(), "tests/fixtures/text_align_start.png");
}

#[test]
fn fixtures_text_align_center() {
  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      width: Percentage(100.0),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(24.0)),
        text_align: Some(TextAlign::Center),
        ..Default::default()
      },
      ..Default::default()
    },
    text: "Center aligned".to_string(),
  };

  run_style_width_test(text.into(), "tests/fixtures/text_align_center.png");
}

#[test]
fn fixtures_text_align_right() {
  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      width: Percentage(100.0),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(24.0)),
        text_align: Some(TextAlign::Right),
        ..Default::default()
      },
      ..Default::default()
    },
    text: "Right aligned".to_string(),
  };

  run_style_width_test(text.into(), "tests/fixtures/text_align_right.png");
}

#[test]
fn fixtures_text_justify_clip() {
  let long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(18.0)),
        line_height: Some(LineHeight(Px(26.0))),
        text_align: Some(TextAlign::Justify),
        text_overflow: Some(TextOverflow::Clip),
        ..Default::default()
      },
      ..Default::default()
    },
    text: long_text.to_string(),
  };

  run_style_width_test(text.into(), "tests/fixtures/text_justify_clip.png");
}

#[test]
fn fixtures_text_ellipsis_line_clamp_2() {
  let long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(18.0)),
        line_height: Some(LineHeight(Px(24.0))),
        text_overflow: Some(TextOverflow::Ellipsis),
        line_clamp: Some(2),
        ..Default::default()
      },
      ..Default::default()
    },
    text: long_text.to_string(),
  };

  run_style_width_test(text.into(), "tests/fixtures/text_ellipsis_line_clamp_2.png");
}

#[test]
fn fixtures_text_transform_all() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      background_color: Some(Color([240, 240, 240, 255])),
      ..Default::default()
    },
    children: Some(vec![
      TextNode {
        style: Style {
          width: Percentage(100.0),
          inheritable_style: InheritableStyle {
            font_size: Some(Px(28.0)),
            text_transform: Some(TextTransform::None),
            ..Default::default()
          },
          ..Default::default()
        },
        text: "None: The quick Brown Fox".to_string(),
      }
      .into(),
      TextNode {
        style: Style {
          width: Percentage(100.0),
          inheritable_style: InheritableStyle {
            font_size: Some(Px(28.0)),
            text_transform: Some(TextTransform::Uppercase),
            ..Default::default()
          },
          ..Default::default()
        },
        text: "Uppercase: The quick Brown Fox".to_string(),
      }
      .into(),
      TextNode {
        style: Style {
          width: Percentage(100.0),
          inheritable_style: InheritableStyle {
            font_size: Some(Px(28.0)),
            text_transform: Some(TextTransform::Lowercase),
            ..Default::default()
          },
          ..Default::default()
        },
        text: "Lowercase: The QUICK Brown FOX".to_string(),
      }
      .into(),
      TextNode {
        style: Style {
          width: Percentage(100.0),
          inheritable_style: InheritableStyle {
            font_size: Some(Px(28.0)),
            text_transform: Some(TextTransform::Capitalize),
            ..Default::default()
          },
          ..Default::default()
        },
        text: "Capitalize: the quick brown fox".to_string(),
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/text_transform_all.png");
}

#[test]
fn fixtures_text_mask_image_gradient_and_emoji() {
  let gradient_images = BackgroundImagesValue::Css(
    "linear-gradient(90deg, #ff3b30, #ffcc00, #34c759, #007aff, #5856d6)".to_string(),
  );

  let text = TextNode {
    style: Style {
      background_color: Some(Color([240, 240, 240, 255])),
      width: Percentage(100.0),
      inheritable_style: InheritableStyle {
        font_size: Some(Px(72.0)),
        ..Default::default()
      },
      mask_image: Some(gradient_images.try_into().unwrap()),
      mask_size: Some(
        BackgroundSizesValue::Css("100% 100%".to_string())
          .try_into()
          .unwrap(),
      ),
      mask_position: Some(
        BackgroundPositionsValue::Css("0 0".to_string())
          .try_into()
          .unwrap(),
      ),
      mask_repeat: Some(
        BackgroundRepeatsValue::Css("no-repeat".to_string())
          .try_into()
          .unwrap(),
      ),
      ..Default::default()
    },
    text: "Gradient Mask Emoji: 🪓 🦊 💩".to_string(),
  };

  run_style_width_test(
    text.clone().into(),
    "tests/fixtures/text_mask_image_gradient_emoji.png",
  );
}
