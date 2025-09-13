use takumi::layout::{
  node::{ContainerNode, NodeKind, TextNode},
  style::{
    BackgroundImagesValue, BackgroundPositionsValue, BackgroundRepeatsValue, BackgroundSizesValue,
    Color, FontWeight,
    LengthUnit::{Percentage, Px},
    LineHeight, StyleBuilder, TextAlign, TextOverflow, TextTransform,
  },
};

mod test_utils;
use test_utils::run_style_width_test;

// Basic text render with defaults
#[test]
fn fixtures_text_basic() {
  let text = TextNode {
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .build()
      .unwrap(),
    text: "The quick brown fox jumps over the lazy dog 12345".to_string(),
  };

  run_style_width_test(NodeKind::Text(text), "tests/fixtures/text_basic.png");
}

#[test]
fn fixtures_text_typography_regular_24px() {
  let text = TextNode {
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .font_size(Px(24.0))
      .build()
      .unwrap(),
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
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .font_size(Px(24.0))
      .font_weight(FontWeight::from(500.0))
      .build()
      .unwrap(),
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
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .font_size(Px(24.0))
      .line_height(LineHeight(Px(40.0)))
      .build()
      .unwrap(),
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
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .font_size(Px(24.0))
      .letter_spacing(Some(Px(2.0)))
      .build()
      .unwrap(),
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
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .width(Percentage(100.0))
      .font_size(Px(24.0))
      .text_align(TextAlign::Start)
      .build()
      .unwrap(),
    text: "Start aligned".to_string(),
  };

  run_style_width_test(text.into(), "tests/fixtures/text_align_start.png");
}

#[test]
fn fixtures_text_align_center() {
  let text = TextNode {
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .width(Percentage(100.0))
      .font_size(Px(24.0))
      .text_align(TextAlign::Center)
      .build()
      .unwrap(),
    text: "Center aligned".to_string(),
  };

  run_style_width_test(text.into(), "tests/fixtures/text_align_center.png");
}

#[test]
fn fixtures_text_align_right() {
  let text = TextNode {
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .width(Percentage(100.0))
      .font_size(Px(24.0))
      .text_align(TextAlign::Right)
      .build()
      .unwrap(),
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
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .font_size(Px(18.0))
      .line_height(LineHeight(Px(26.0)))
      .text_align(TextAlign::Justify)
      .text_overflow(TextOverflow::Clip)
      .build()
      .unwrap(),
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
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .font_size(Px(18.0))
      .line_height(LineHeight(Px(24.0)))
      .text_overflow(TextOverflow::Ellipsis)
      .line_clamp(Some(2))
      .build()
      .unwrap(),
    text: long_text.to_string(),
  };

  run_style_width_test(text.into(), "tests/fixtures/text_ellipsis_line_clamp_2.png");
}

#[test]
fn fixtures_text_transform_all() {
  let container = ContainerNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .background_color(Color([240, 240, 240, 255]))
      .build()
      .unwrap(),
    children: Some(vec![
      TextNode {
        style: StyleBuilder::default()
          .width(Percentage(100.0))
          .font_size(Px(28.0))
          .text_transform(TextTransform::None)
          .build()
          .unwrap(),
        text: "None: The quick Brown Fox".to_string(),
      }
      .into(),
      TextNode {
        style: StyleBuilder::default()
          .width(Percentage(100.0))
          .font_size(Px(28.0))
          .text_transform(TextTransform::Uppercase)
          .build()
          .unwrap(),
        text: "Uppercase: The quick Brown Fox".to_string(),
      }
      .into(),
      TextNode {
        style: StyleBuilder::default()
          .width(Percentage(100.0))
          .font_size(Px(28.0))
          .text_transform(TextTransform::Lowercase)
          .build()
          .unwrap(),
        text: "Lowercase: The QUICK Brown FOX".to_string(),
      }
      .into(),
      TextNode {
        style: StyleBuilder::default()
          .width(Percentage(100.0))
          .font_size(Px(28.0))
          .text_transform(TextTransform::Capitalize)
          .build()
          .unwrap(),
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
    style: StyleBuilder::default()
      .background_color(Color([240, 240, 240, 255]))
      .width(Percentage(100.0))
      .font_size(Px(72.0))
      .mask_image(Some(gradient_images.try_into().unwrap()))
      .mask_size(Some(
        BackgroundSizesValue::Css("100% 100%".to_string())
          .try_into()
          .unwrap(),
      ))
      .mask_position(Some(
        BackgroundPositionsValue::Css("0 0".to_string())
          .try_into()
          .unwrap(),
      ))
      .mask_repeat(Some(
        BackgroundRepeatsValue::Css("no-repeat".to_string())
          .try_into()
          .unwrap(),
      ))
      .build()
      .unwrap(),
    text: "Gradient Mask Emoji: 🪓 🦊 💩".to_string(),
  };

  run_style_width_test(
    text.clone().into(),
    "tests/fixtures/text_mask_image_gradient_emoji.png",
  );
}
