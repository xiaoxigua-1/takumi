use std::sync::LazyLock;

use takumi::{GlobalContext, resources::font::FontError};

// Include test font data using include_bytes!
static TTF_FONT: &[u8] = include_bytes!("../../assets/fonts/noto-sans/NotoSans-Regular.ttf");
static WOFF_FONT: &[u8] = include_bytes!("../../assets/fonts/noto-sans/NotoSansTC-Bold.woff");
static WOFF2_FONT: &[u8] =
  include_bytes!("../../assets/fonts/sil/scheherazade-new-v17-arabic-regular.woff2");

static CONTEXT: LazyLock<GlobalContext> = LazyLock::new(GlobalContext::default);

#[test]
fn test_ttf_font_loading() {
  assert!(
    CONTEXT
      .font_context
      .load_and_store(TTF_FONT.to_vec())
      .is_ok()
  );
}

#[test]
fn test_woff_font_loading() {
  assert!(
    CONTEXT
      .font_context
      .load_and_store(WOFF_FONT.to_vec())
      .is_ok()
  );
}

#[test]
fn test_woff2_font_loading() {
  assert!(
    CONTEXT
      .font_context
      .load_and_store(WOFF2_FONT.to_vec())
      .is_ok()
  );
}

#[test]
fn test_invalid_format_detection() {
  // Test with invalid data
  let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
  let result = CONTEXT.font_context.load_and_store(invalid_data);
  assert!(matches!(result, Err(FontError::UnsupportedFormat)));
}

#[test]
fn test_empty_data() {
  // Test with empty data
  let empty_data = vec![];
  let result = CONTEXT.font_context.load_and_store(empty_data);
  assert!(matches!(result, Err(FontError::UnsupportedFormat)));
}

#[test]
fn test_too_short_data() {
  // Test with data too short for format detection
  let short_data = vec![0x00, 0x01, 0x00];
  let result = CONTEXT.font_context.load_and_store(short_data);
  assert!(matches!(result, Err(FontError::UnsupportedFormat)));
}
