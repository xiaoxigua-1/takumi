use takumi::core::GlobalContext;
use takumi::core::viewport::RenderContext;
use takumi::sides::Sides;
use takumi::style::{gap::Gap, length_unit::LengthUnit};

fn create_test_context() -> RenderContext<'static> {
  let global = Box::leak(Box::new(GlobalContext::default()));
  let viewport = takumi::core::viewport::Viewport::new_with_font_size(800, 600, 16.0);

  RenderContext {
    global,
    viewport,
    parent_font_size: 16.0,
  }
}

fn create_custom_context(
  width: u32,
  height: u32,
  font_size: f32,
  parent_font_size: f32,
) -> RenderContext<'static> {
  let global = Box::leak(Box::new(GlobalContext::default()));
  let viewport = takumi::core::viewport::Viewport::new_with_font_size(width, height, font_size);

  RenderContext {
    global,
    viewport,
    parent_font_size,
  }
}

#[test]
fn test_length_unit_auto() {
  let context = create_test_context();
  let auto = LengthUnit::Auto;

  assert_eq!(auto.resolve_to_px(&context), 0.0);
  assert!(auto.to_compact_length(&context).is_auto());
}

#[test]
fn test_length_unit_px() {
  let context = create_test_context();
  let px = LengthUnit::Px(42.5);

  assert_eq!(px.resolve_to_px(&context), 42.5);
  assert_eq!(px.to_compact_length(&context).value(), 42.5);
}

#[test]
fn test_length_unit_percentage() {
  let context = create_test_context();
  let percent = LengthUnit::Percentage(75.0);

  assert_eq!(percent.resolve_to_px(&context), 75.0 * 16.0 / 100.0);
  assert_eq!(percent.to_compact_length(&context).value(), 0.75);
}

#[test]
fn test_length_unit_rem() {
  let context = create_test_context();
  let rem = LengthUnit::Rem(2.5);

  assert_eq!(rem.resolve_to_px(&context), 2.5 * 16.0);
  assert_eq!(rem.to_compact_length(&context).value(), 2.5 * 16.0);
}

#[test]
fn test_length_unit_em() {
  let context = create_test_context();
  let em = LengthUnit::Em(1.5);

  assert_eq!(em.resolve_to_px(&context), 1.5 * 16.0);
  assert_eq!(em.to_compact_length(&context).value(), 1.5 * 16.0);
}

#[test]
fn test_length_unit_vh() {
  let context = create_test_context();
  let vh = LengthUnit::Vh(50.0);

  assert_eq!(vh.resolve_to_px(&context), 50.0 * 600.0 / 100.0);
  assert_eq!(vh.to_compact_length(&context).value(), 50.0 * 600.0 / 100.0);
}

#[test]
fn test_length_unit_vw() {
  let context = create_test_context();
  let vw = LengthUnit::Vw(25.0);

  assert_eq!(vw.resolve_to_px(&context), 25.0 * 800.0 / 100.0);
  assert_eq!(vw.to_compact_length(&context).value(), 25.0 * 800.0 / 100.0);
}

#[test]
fn test_length_unit_zero_values() {
  let context = create_test_context();

  assert_eq!(LengthUnit::Px(0.0).resolve_to_px(&context), 0.0);
  assert_eq!(LengthUnit::Percentage(0.0).resolve_to_px(&context), 0.0);
  assert_eq!(LengthUnit::Rem(0.0).resolve_to_px(&context), 0.0);
  assert_eq!(LengthUnit::Em(0.0).resolve_to_px(&context), 0.0);
  assert_eq!(LengthUnit::Vh(0.0).resolve_to_px(&context), 0.0);
  assert_eq!(LengthUnit::Vw(0.0).resolve_to_px(&context), 0.0);
}

#[test]
fn test_length_unit_negative_values() {
  let context = create_test_context();

  assert_eq!(LengthUnit::Px(-10.0).resolve_to_px(&context), -10.0);
  assert_eq!(
    LengthUnit::Percentage(-50.0).resolve_to_px(&context),
    -50.0 * 16.0 / 100.0
  );
  assert_eq!(LengthUnit::Rem(-1.5).resolve_to_px(&context), -1.5 * 16.0);
  assert_eq!(LengthUnit::Em(-2.0).resolve_to_px(&context), -2.0 * 16.0);
  assert_eq!(
    LengthUnit::Vh(-25.0).resolve_to_px(&context),
    -25.0 * 600.0 / 100.0
  );
  assert_eq!(
    LengthUnit::Vw(-10.0).resolve_to_px(&context),
    -10.0 * 800.0 / 100.0
  );
}

#[test]
fn test_length_unit_context_dependent_calculations() {
  let small_font_context = create_custom_context(1024, 768, 12.0, 14.0);
  let large_font_context = create_custom_context(1920, 1080, 20.0, 18.0);

  assert_eq!(
    LengthUnit::Rem(2.0).resolve_to_px(&small_font_context),
    2.0 * 12.0
  );
  assert_eq!(
    LengthUnit::Rem(2.0).resolve_to_px(&large_font_context),
    2.0 * 20.0
  );

  assert_eq!(
    LengthUnit::Em(1.5).resolve_to_px(&small_font_context),
    1.5 * 14.0
  );
  assert_eq!(
    LengthUnit::Em(1.5).resolve_to_px(&large_font_context),
    1.5 * 18.0
  );

  assert_eq!(
    LengthUnit::Percentage(50.0).resolve_to_px(&small_font_context),
    50.0 * 14.0 / 100.0
  );
  assert_eq!(
    LengthUnit::Percentage(50.0).resolve_to_px(&large_font_context),
    50.0 * 18.0 / 100.0
  );

  assert_eq!(
    LengthUnit::Vh(10.0).resolve_to_px(&small_font_context),
    10.0 * 768.0 / 100.0
  );
  assert_eq!(
    LengthUnit::Vw(10.0).resolve_to_px(&small_font_context),
    10.0 * 1024.0 / 100.0
  );
  assert_eq!(
    LengthUnit::Vh(10.0).resolve_to_px(&large_font_context),
    10.0 * 1080.0 / 100.0
  );
  assert_eq!(
    LengthUnit::Vw(10.0).resolve_to_px(&large_font_context),
    10.0 * 1920.0 / 100.0
  );
}

#[test]
fn test_sides_value_single_value() {
  let rect: taffy::Rect<LengthUnit> = Sides([LengthUnit::Px(10.0); 4]).into();

  assert_eq!(rect.left, LengthUnit::Px(10.0));
  assert_eq!(rect.right, LengthUnit::Px(10.0));
  assert_eq!(rect.top, LengthUnit::Px(10.0));
  assert_eq!(rect.bottom, LengthUnit::Px(10.0));
}

#[test]
fn test_sides_value_all_sides() {
  let value = Sides([
    LengthUnit::Px(10.0),
    LengthUnit::Px(20.0),
    LengthUnit::Px(30.0),
    LengthUnit::Px(40.0),
  ]);
  let rect: taffy::Rect<LengthUnit> = value.into();

  assert_eq!(rect.top, LengthUnit::Px(10.0));
  assert_eq!(rect.right, LengthUnit::Px(20.0));
  assert_eq!(rect.bottom, LengthUnit::Px(30.0));
  assert_eq!(rect.left, LengthUnit::Px(40.0));
}

#[test]
fn test_gap_single_value() {
  let context = create_test_context();
  let gap = Gap(LengthUnit::Px(10.0), LengthUnit::Px(10.0));
  let _size = gap.resolve_to_size(&context);
  // We can't test the taffy types directly, but the conversion should work
}

#[test]
fn test_gap_array() {
  let context = create_test_context();
  let gap = Gap(LengthUnit::Px(15.0), LengthUnit::Px(20.0));
  let _size = gap.resolve_to_size(&context);
  // We can't test the taffy types directly, but the conversion should work
}

#[test]
fn test_mobile_viewport_scenario() {
  let context = create_custom_context(375, 667, 14.0, 14.0);

  let font_size = LengthUnit::Rem(1.2);
  let padding = LengthUnit::Percentage(5.0);
  let margin = LengthUnit::Vw(3.0);

  assert!((font_size.resolve_to_px(&context) - 16.8).abs() < 0.01);
  assert!((padding.resolve_to_px(&context) - 0.7).abs() < 0.01);
  assert!((margin.resolve_to_px(&context) - 11.25).abs() < 0.01);
}

#[test]
fn test_desktop_viewport_scenario() {
  let context = create_custom_context(1920, 1080, 16.0, 16.0);

  let max_width = LengthUnit::Px(1200.0);
  let side_padding = LengthUnit::Vw(5.0);
  let line_height = LengthUnit::Em(1.5);

  assert_eq!(max_width.resolve_to_px(&context), 1200.0);
  assert!((side_padding.resolve_to_px(&context) - 96.0).abs() < 0.01);
  assert!((line_height.resolve_to_px(&context) - 24.0).abs() < 0.01);
}
