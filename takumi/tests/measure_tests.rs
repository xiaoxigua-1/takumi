use std::sync::LazyLock;

use cosmic_text::Weight;
use taffy::{AvailableSpace, geometry::Size};
use takumi::{
  GlobalContext,
  layout::{
    DEFAULT_FONT_SIZE, DEFAULT_LINE_HEIGHT_SCALER, Viewport,
    style::{Color, FontFamily, FontStyle, TextOverflow, TextStyle, TextTransform},
  },
  rendering::{DEFAULT_SCALE, RenderContext},
};

const NOTO_SANS_REGULAR_BUFFER: &[u8] =
  include_bytes!("../../assets/fonts/noto-sans/NotoSans-Regular.ttf");

// Viewport dimensions
const VIEWPORT_WIDTH: u32 = 800;
const VIEWPORT_HEIGHT: u32 = 600;

// Font settings
const LETTER_SPACING_0: f32 = 0.0;
const FONT_FAMILY_NOTO_SANS: &str = "NotoSans";

static SHARED_GLOBAL_CONTEXT: LazyLock<GlobalContext> = LazyLock::new(|| {
  let global_context = GlobalContext::default();

  global_context
    .font_context
    .load_and_store(NOTO_SANS_REGULAR_BUFFER.to_vec())
    .unwrap();

  global_context
});

// Helper function to create a basic RenderContext for testing
fn create_test_context() -> RenderContext<'static> {
  RenderContext {
    global: &SHARED_GLOBAL_CONTEXT,
    parent_font_size: DEFAULT_FONT_SIZE,
    viewport: Viewport {
      width: VIEWPORT_WIDTH,
      height: VIEWPORT_HEIGHT,
      font_size: DEFAULT_FONT_SIZE,
    },
    scale: DEFAULT_SCALE,
  }
}

// Helper function to create a basic ResolvedFontStyle for testing
fn create_test_font_style() -> FontStyle {
  FontStyle {
    font_size: DEFAULT_FONT_SIZE,
    line_height: DEFAULT_FONT_SIZE * DEFAULT_LINE_HEIGHT_SCALER,
    font_weight: Weight::NORMAL,
    text_style: TextStyle::Normal,
    line_clamp: None,
    font_family: Some(FontFamily::Custom(FONT_FAMILY_NOTO_SANS.to_string()).into()),
    letter_spacing: Some(LETTER_SPACING_0),
    text_align: Some(cosmic_text::Align::Left),
    text_overflow: TextOverflow::Clip,
    text_transform: TextTransform::None,
    color: Color::black(),
  }
}

// Helper function to create image size for testing
fn create_image_size(width: f32, height: f32) -> Size<f32> {
  Size { width, height }
}

// Helper function to create known dimensions for testing
fn create_known_dimensions(width: Option<f32>, height: Option<f32>) -> Size<Option<f32>> {
  Size { width, height }
}

// Helper function to create available space for testing
fn create_available_space(width: AvailableSpace, height: AvailableSpace) -> Size<AvailableSpace> {
  Size { width, height }
}

mod measure_image_tests {
  use takumi::layout::node::measure_image;

  use super::*;

  #[test]
  fn test_measure_image_with_known_dimensions() {
    let result = measure_image(
      create_image_size(800.0, 600.0),
      create_known_dimensions(Some(400.0), Some(300.0)),
      create_available_space(
        AvailableSpace::Definite(500.0),
        AvailableSpace::Definite(400.0),
      ),
    );

    assert_eq!(result.width, 400.0);
    assert_eq!(result.height, 300.0);
  }

  #[test]
  fn test_measure_image_with_width_only() {
    let result = measure_image(
      create_image_size(800.0, 600.0),
      create_known_dimensions(Some(400.0), None),
      create_available_space(
        AvailableSpace::Definite(500.0),
        AvailableSpace::Definite(400.0),
      ),
    );

    assert_eq!(result.width, 400.0);
    assert_eq!(result.height, 300.0); // 400 / (800/600) = 300
  }

  #[test]
  fn test_measure_image_with_height_only() {
    let result = measure_image(
      create_image_size(800.0, 600.0),
      create_known_dimensions(None, Some(300.0)),
      create_available_space(
        AvailableSpace::Definite(500.0),
        AvailableSpace::Definite(400.0),
      ),
    );

    assert_eq!(result.width, 400.0); // 300 * (800/600) = 400
    assert_eq!(result.height, 300.0);
  }

  #[test]
  fn test_measure_image_with_available_space_only() {
    let result = measure_image(
      create_image_size(800.0, 600.0),
      create_known_dimensions(None, None),
      create_available_space(
        AvailableSpace::Definite(400.0),
        AvailableSpace::Definite(300.0),
      ),
    );

    // Should respect available space constraints
    assert_eq!(result.width, 400.0);
    assert_eq!(result.height, 300.0);
  }

  #[test]
  fn test_measure_image_with_no_constraints() {
    let result = measure_image(
      create_image_size(800.0, 600.0),
      create_known_dimensions(None, None),
      create_available_space(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
    );

    assert_eq!(result.width, 800.0);
    assert_eq!(result.height, 600.0);
  }

  #[test]
  fn test_measure_image_square_aspect_ratio() {
    let result = measure_image(
      create_image_size(500.0, 500.0),
      create_known_dimensions(Some(200.0), None),
      create_available_space(
        AvailableSpace::Definite(300.0),
        AvailableSpace::Definite(300.0),
      ),
    );

    assert_eq!(result.width, 200.0);
    assert_eq!(result.height, 200.0); // Square aspect ratio
  }

  #[test]
  fn test_measure_image_extreme_aspect_ratio() {
    let result = measure_image(
      create_image_size(2000.0, 100.0), // 20:1 aspect ratio
      create_known_dimensions(Some(400.0), None),
      create_available_space(
        AvailableSpace::Definite(500.0),
        AvailableSpace::Definite(500.0),
      ),
    );

    assert_eq!(result.width, 400.0);
    assert_eq!(result.height, 20.0); // 400 / 20 = 20
  }

  #[test]
  fn test_measure_image_zero_dimensions() {
    let result = measure_image(
      create_image_size(0.0, 0.0),
      create_known_dimensions(Some(100.0), None),
      create_available_space(
        AvailableSpace::Definite(200.0),
        AvailableSpace::Definite(200.0),
      ),
    );

    // When source has zero dimensions, aspect ratio calculation would be NaN
    // The function should handle this gracefully
    assert_eq!(result.width, 100.0);
    assert!(result.height.is_finite());
  }

  #[test]
  fn test_measure_image_very_small_dimensions() {
    let result = measure_image(
      create_image_size(1.0, 1.0),
      create_known_dimensions(None, Some(100.0)),
      create_available_space(
        AvailableSpace::Definite(200.0),
        AvailableSpace::Definite(200.0),
      ),
    );

    assert_eq!(result.width, 100.0);
    assert_eq!(result.height, 100.0);
  }

  #[test]
  fn test_measure_image_partial_available_space() {
    let result = measure_image(
      create_image_size(800.0, 600.0),
      create_known_dimensions(None, None),
      create_available_space(AvailableSpace::Definite(400.0), AvailableSpace::MaxContent),
    );

    // Should use available width and original height
    assert_eq!(result.width, 400.0);
    assert_eq!(result.height, 300.0); // Since max-content is applied, height should be capped
  }
}

mod measure_text_tests {
  use super::*;
  use takumi::layout::node::measure_text;

  // Helper function to create text measurement parameters
  fn measure_text_helper(
    text: &str,
    width: Option<f32>,
    height: Option<f32>,
    available_width: AvailableSpace,
    available_height: AvailableSpace,
  ) -> Size<f32> {
    let context = create_test_context();
    let style = create_test_font_style();

    measure_text(
      context.global,
      text,
      &style,
      create_known_dimensions(width, height),
      create_available_space(available_width, available_height),
    )
  }

  // Helper function to measure text with custom style
  fn measure_text_with_style(
    text: &str,
    style: &FontStyle,
    width: Option<f32>,
    height: Option<f32>,
    available_width: AvailableSpace,
    available_height: AvailableSpace,
  ) -> Size<f32> {
    let context = create_test_context();

    measure_text(
      context.global,
      text,
      style,
      create_known_dimensions(width, height),
      create_available_space(available_width, available_height),
    )
  }

  #[test]
  fn test_measure_text_basic() {
    let result = measure_text_helper(
      "Hello, world!",
      None,
      None,
      AvailableSpace::MaxContent,
      AvailableSpace::MaxContent,
    );

    assert!(result.width > 0.0);
    assert_eq!(
      result.height,
      (DEFAULT_FONT_SIZE * DEFAULT_LINE_HEIGHT_SCALER).ceil()
    );
  }

  #[test]
  fn test_measure_text_with_width_constraint() {
    let result = measure_text_helper(
      "This is a long text that should wrap when given a width constraint",
      Some(200.0),
      None,
      AvailableSpace::Definite(300.0),
      AvailableSpace::MaxContent,
    );

    assert!(result.width <= 200.0);
    assert!(result.height >= 2.0 * DEFAULT_FONT_SIZE * DEFAULT_LINE_HEIGHT_SCALER);
    // Have to allow one pixel tolerance due to rounding
    assert!((result.height % (DEFAULT_FONT_SIZE * DEFAULT_LINE_HEIGHT_SCALER)).floor() == 0.0);
  }

  #[test]
  fn test_measure_text_with_height_constraint() {
    let mut style = create_test_font_style();
    style.line_clamp = Some(2); // Limit to 2 lines

    let result = measure_text_with_style(
      "This is a long text that should be clamped to a specific number of lines",
      &style,
      Some(200.0),
      None,
      AvailableSpace::Definite(300.0),
      AvailableSpace::MaxContent,
    );

    let expected_height = (2.0 * style.line_height).ceil();
    assert_eq!(result.height, expected_height);
  }

  #[test]
  fn test_measure_text_empty_string() {
    let result = measure_text_helper(
      "",
      None,
      None,
      AvailableSpace::MaxContent,
      AvailableSpace::MaxContent,
    );

    assert_eq!(result.width, 0.0);
    assert_eq!(result.height, 0.0);
  }

  #[test]
  fn test_measure_text_single_character() {
    let result = measure_text_helper(
      "A",
      None,
      None,
      AvailableSpace::MaxContent,
      AvailableSpace::MaxContent,
    );

    assert!(result.width > 0.0);
    assert_eq!(
      result.height,
      (DEFAULT_FONT_SIZE * DEFAULT_LINE_HEIGHT_SCALER).ceil()
    );
  }

  #[test]
  fn test_measure_text_with_line_clamp() {
    let mut style = create_test_font_style();
    style.line_clamp = Some(3);

    let result = measure_text_with_style(
      "Line 1\nLine 2\nLine 3\nLine 4\nLine 5",
      &style,
      Some(300.0),
      None,
      AvailableSpace::Definite(400.0),
      AvailableSpace::MaxContent,
    );

    let expected_height = (3.0 * style.line_height).ceil();
    assert_eq!(result.height, expected_height);
  }

  #[test]
  fn test_measure_text_zero_width_constraint() {
    let result = measure_text_helper(
      "This text has zero width constraint",
      Some(0.0),
      None,
      AvailableSpace::MinContent,
      AvailableSpace::MaxContent,
    );

    // With zero width, the text should not be rendered
    assert_eq!(result.width, 0.0);
    assert_eq!(result.height, 0.0);
  }

  #[test]
  fn test_measure_text_zero_height_constraint() {
    let result = measure_text_helper(
      "This text has zero height constraint",
      Some(200.0),
      Some(0.0),
      AvailableSpace::Definite(300.0),
      AvailableSpace::MinContent,
    );

    // With zero height, the text should not be rendered
    assert_eq!(result.width, 0.0);
    assert_eq!(result.height, 0.0);
  }

  #[test]
  fn test_measure_text_with_different_font_size() {
    let mut style = create_test_font_style();
    style.font_size = 24.0;
    style.line_height = 24.0 * DEFAULT_LINE_HEIGHT_SCALER;

    let result = measure_text_with_style(
      "Large font text",
      &style,
      None,
      None,
      AvailableSpace::MaxContent,
      AvailableSpace::MaxContent,
    );

    assert!(result.width > 0.0);
    assert_eq!(result.height, style.line_height.ceil());
  }

  #[test]
  fn test_measure_text_with_different_line_height() {
    let mut style = create_test_font_style();
    style.line_height = DEFAULT_FONT_SIZE * 1.5;

    let result = measure_text_with_style(
      "Text with increased line height",
      &style,
      None,
      None,
      AvailableSpace::MaxContent,
      AvailableSpace::MaxContent,
    );

    assert!(result.width > 0.0);
    assert_eq!(result.height, style.line_height);
  }

  #[test]
  fn test_measure_text_whitespace_only() {
    let result = measure_text_helper(
      "   \n\t  \n  ",
      Some(200.0),
      None,
      AvailableSpace::Definite(300.0),
      AvailableSpace::MaxContent,
    );

    assert!(result.width >= 0.0);
    assert!(result.height >= 0.0);
  }

  #[test]
  fn test_measure_text_very_long_word() {
    let result = measure_text_helper(
      "verylongwordwithoutanyspaces",
      Some(100.0),
      None,
      AvailableSpace::Definite(200.0),
      AvailableSpace::MaxContent,
    );

    assert!(result.width <= 100.0);
    assert!(result.height >= 19.0);
  }
}
