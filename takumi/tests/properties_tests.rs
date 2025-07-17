use takumi::style::properties::*;
use takumi::style::units::Gap;
use takumi::style::{Color, ColorInput, LengthUnit, SidesValue};

#[test]
fn test_style_display() {
  let style = Style {
    display: Display::Flex,
    ..Default::default()
  };
  assert_eq!(style.display, Display::Flex);
}

#[test]
fn test_style_flex_direction() {
  let style = Style {
    flex_direction: FlexDirection::Column,
    ..Default::default()
  };
  assert_eq!(style.flex_direction, FlexDirection::Column);
}

#[test]
fn test_style_flex_wrap() {
  let style = Style {
    flex_wrap: FlexWrap::Wrap,
    ..Default::default()
  };
  assert_eq!(style.flex_wrap, FlexWrap::Wrap);
}

#[test]
fn test_style_justify_content() {
  let style = Style {
    justify_content: Some(JustifyContent::Center),
    ..Default::default()
  };
  assert_eq!(style.justify_content, Some(JustifyContent::Center));
}

#[test]
fn test_style_align_items() {
  let style = Style {
    align_items: Some(AlignItems::Center),
    ..Default::default()
  };
  assert_eq!(style.align_items, Some(AlignItems::Center));
}

#[test]
fn test_style_position() {
  let style = Style {
    position: Position::Absolute,
    ..Default::default()
  };
  assert_eq!(style.position, Position::Absolute);
}

#[test]
fn test_style_object_fit() {
  let style = Style {
    object_fit: ObjectFit::Cover,
    ..Default::default()
  };
  assert_eq!(style.object_fit, ObjectFit::Cover);
}

#[test]
fn test_style_text_overflow() {
  let mut style = Style::default();
  style.inheritable_style.text_overflow = Some(TextOverflow::Ellipsis);
  assert_eq!(
    style.inheritable_style.text_overflow,
    Some(TextOverflow::Ellipsis)
  );
}

#[test]
fn test_style_text_align() {
  let mut style = Style::default();
  style.inheritable_style.text_align = Some(TextAlign::Center);
  assert_eq!(style.inheritable_style.text_align, Some(TextAlign::Center));
}

#[test]
fn test_style_background_color() {
  let mut style = Style::default();
  let color = ColorInput::Color(Color::Rgba(255, 0, 0, 1.0));
  style.background_color = Some(color.clone());
  assert_eq!(style.background_color, Some(color));
}

#[test]
fn test_style_color() {
  let mut style = Style::default();
  let color = ColorInput::Color(Color::Rgba(0, 255, 0, 1.0));
  style.inheritable_style.color = Some(color.clone());
  assert_eq!(style.inheritable_style.color, Some(color));
}

#[test]
fn test_style_border_color() {
  let mut style = Style::default();
  let color = ColorInput::Color(Color::Rgba(0, 0, 255, 1.0));
  style.inheritable_style.border_color = Some(color.clone());
  assert_eq!(style.inheritable_style.border_color, Some(color));
}

#[test]
fn test_style_box_shadow() {
  let mut style = Style::default();
  let shadow = BoxShadow {
    offset_x: LengthUnit::Px(2.0),
    offset_y: LengthUnit::Px(2.0),
    blur_radius: LengthUnit::Px(4.0),
    spread_radius: LengthUnit::Px(0.0),
    color: ColorInput::Color(Color::Rgba(0, 0, 0, 0.3)),
    inset: false,
  };
  style.box_shadow = Some(BoxShadowInput::Single(shadow.clone()));
  assert_eq!(style.box_shadow, Some(BoxShadowInput::Single(shadow)));
}

#[test]
fn test_style_width_height() {
  let style = Style {
    width: LengthUnit::Px(100.0),
    height: LengthUnit::Px(200.0),
    ..Default::default()
  };
  assert_eq!(style.width, LengthUnit::Px(100.0));
  assert_eq!(style.height, LengthUnit::Px(200.0));
}

#[test]
fn test_style_margin_padding() {
  let mut style = Style::default();
  let margin = SidesValue::SingleValue(LengthUnit::Px(10.0));
  let padding = SidesValue::SingleValue(LengthUnit::Px(5.0));
  style.margin = margin;
  style.padding = padding;
  assert_eq!(style.margin, margin);
  assert_eq!(style.padding, padding);
}

#[test]
fn test_style_border_radius() {
  let mut style = Style::default();
  let radius = SidesValue::SingleValue(LengthUnit::Px(8.0));
  style.inheritable_style.border_radius = Some(radius);
  assert_eq!(style.inheritable_style.border_radius, Some(radius));
}

#[test]
fn test_style_border_width() {
  let mut style = Style::default();
  let width = SidesValue::SingleValue(LengthUnit::Px(2.0));
  style.border_width = width;
  assert_eq!(style.border_width, width);
}

#[test]
fn test_style_flex_grow_shrink_basis() {
  let style = Style {
    flex_grow: 2.0,
    flex_shrink: 0.5,
    flex_basis: LengthUnit::Px(100.0),
    ..Default::default()
  };
  assert_eq!(style.flex_grow, 2.0);
  assert_eq!(style.flex_shrink, 0.5);
  assert_eq!(style.flex_basis, LengthUnit::Px(100.0));
}

#[test]
fn test_style_gap() {
  let mut style = Style::default();
  let gap = Gap::Array(LengthUnit::Px(10.0), LengthUnit::Px(20.0));
  style.gap = gap;
  assert_eq!(style.gap, gap);
}

#[test]
fn test_style_inset() {
  let mut style = Style::default();
  let inset = SidesValue::SingleValue(LengthUnit::Px(5.0));
  style.inset = inset;
  assert_eq!(style.inset, inset);
}

#[test]
fn test_style_min_max_width_height() {
  let style = Style {
    min_width: LengthUnit::Px(100.0),
    min_height: LengthUnit::Px(50.0),
    max_width: LengthUnit::Px(500.0),
    max_height: LengthUnit::Px(300.0),
    ..Default::default()
  };
  assert_eq!(style.min_width, LengthUnit::Px(100.0));
  assert_eq!(style.min_height, LengthUnit::Px(50.0));
  assert_eq!(style.max_width, LengthUnit::Px(500.0));
  assert_eq!(style.max_height, LengthUnit::Px(300.0));
}

#[test]
fn test_style_aspect_ratio() {
  let style = Style {
    aspect_ratio: Some(1.5),
    ..Default::default()
  };
  assert_eq!(style.aspect_ratio, Some(1.5));
}

#[test]
fn test_style_font_size() {
  let mut style = Style::default();
  style.inheritable_style.font_size = Some(LengthUnit::Px(16.0));
  assert_eq!(
    style.inheritable_style.font_size,
    Some(LengthUnit::Px(16.0))
  );
}

#[test]
fn test_style_line_height() {
  let mut style = Style::default();
  style.inheritable_style.line_height = Some(LengthUnit::Px(1.5));
  assert_eq!(
    style.inheritable_style.line_height,
    Some(LengthUnit::Px(1.5))
  );
}

#[test]
fn test_style_letter_spacing() {
  let mut style = Style::default();
  style.inheritable_style.letter_spacing = Some(LengthUnit::Em(1.0));
  assert_eq!(
    style.inheritable_style.letter_spacing,
    Some(LengthUnit::Em(1.0))
  );
}

#[test]
fn test_style_font_weight() {
  let mut style = Style::default();
  style.inheritable_style.font_weight = Some(FontWeight(700));
  assert_eq!(style.inheritable_style.font_weight, Some(FontWeight(700)));
}

#[test]
fn test_style_font_family() {
  let mut style = Style::default();
  style.inheritable_style.font_family = Some("Arial".to_string());
  assert_eq!(
    style.inheritable_style.font_family,
    Some("Arial".to_string())
  );
}

#[test]
fn test_style_line_clamp() {
  let mut style = Style::default();
  style.inheritable_style.line_clamp = Some(3);
  assert_eq!(style.inheritable_style.line_clamp, Some(3));
}

// These tests are commented out as z_index and opacity are not direct fields on Style
// They might be part of inheritable_style or another structure

#[test]
fn test_style_grid_template_columns_rows() {
  let mut style = Style::default();
  let template = vec![TrackSizingFunction::Single(GridTrackSize::Fr(1.0))];
  style.grid_template_columns = Some(template.clone());
  style.grid_template_rows = Some(template.clone());
  assert_eq!(style.grid_template_columns, Some(template.clone()));
  assert_eq!(style.grid_template_rows, Some(template));
}

#[test]
fn test_style_grid_auto_columns_rows_flow() {
  let style = Style {
    grid_auto_columns: Some(vec![GridTrackSize::Fr(1.0)]),
    grid_auto_rows: Some(vec![GridTrackSize::Fr(1.0)]),
    grid_auto_flow: Some(GridAutoFlow::Column),
    ..Default::default()
  };
  assert_eq!(style.grid_auto_columns, Some(vec![GridTrackSize::Fr(1.0)]));
  assert_eq!(style.grid_auto_rows, Some(vec![GridTrackSize::Fr(1.0)]));
  assert_eq!(style.grid_auto_flow, Some(GridAutoFlow::Column));
}

#[test]
fn test_style_grid_column_row() {
  let mut style = Style::default();

  let grid_column = GridLine {
    start: Some(GridPlacement::Line(1)),
    end: Some(GridPlacement::Line(3)),
  };

  let grid_row = GridLine {
    start: Some(GridPlacement::Line(2)),
    end: Some(GridPlacement::Line(4)),
  };

  style.grid_column = Some(grid_column);
  style.grid_row = Some(grid_row);

  assert_eq!(
    style.grid_column,
    Some(GridLine {
      start: Some(GridPlacement::Line(1)),
      end: Some(GridPlacement::Line(3)),
    })
  );
  assert_eq!(
    style.grid_row,
    Some(GridLine {
      start: Some(GridPlacement::Line(2)),
      end: Some(GridPlacement::Line(4)),
    })
  );
}

#[test]
fn test_inheritable_style_color() {
  let mut style = InheritableStyle::default();
  let color = ColorInput::Color(Color::Rgba(255, 0, 0, 1.0));
  style.color = Some(color.clone());
  assert_eq!(style.color, Some(color));
}

#[test]
fn test_inheritable_style_font_size() {
  let style = InheritableStyle {
    font_size: Some(LengthUnit::Px(16.0)),
    ..Default::default()
  };
  assert_eq!(style.font_size, Some(LengthUnit::Px(16.0)));
}

#[test]
fn test_inheritable_style_line_height() {
  let style = InheritableStyle {
    line_height: Some(LengthUnit::Px(1.5)),
    ..Default::default()
  };
  assert_eq!(style.line_height, Some(LengthUnit::Px(1.5)));
}

#[test]
fn test_inheritable_style_letter_spacing() {
  let style = InheritableStyle {
    letter_spacing: Some(LengthUnit::Em(1.0)),
    ..Default::default()
  };
  assert_eq!(style.letter_spacing, Some(LengthUnit::Em(1.0)));
}

#[test]
fn test_inheritable_style_font_weight() {
  let style = InheritableStyle {
    font_weight: Some(FontWeight(700)),
    ..Default::default()
  };
  assert_eq!(style.font_weight, Some(FontWeight(700)));
}

#[test]
fn test_inheritable_style_font_family() {
  let style = InheritableStyle {
    font_family: Some("Arial".to_string()),
    ..Default::default()
  };
  assert_eq!(style.font_family, Some("Arial".to_string()));
}

#[test]
fn test_inheritable_style_line_clamp() {
  let style = InheritableStyle {
    line_clamp: Some(3),
    ..Default::default()
  };
  assert_eq!(style.line_clamp, Some(3));
}

#[test]
fn test_inheritable_style_text_overflow() {
  let style = InheritableStyle {
    text_overflow: Some(TextOverflow::Ellipsis),
    ..Default::default()
  };
  assert_eq!(style.text_overflow, Some(TextOverflow::Ellipsis));
}

#[test]
fn test_inheritable_style_text_align() {
  let style = InheritableStyle {
    text_align: Some(TextAlign::Center),
    ..Default::default()
  };
  assert_eq!(style.text_align, Some(TextAlign::Center));
}

#[test]
fn test_inheritable_style_border_color() {
  let mut style = InheritableStyle::default();
  let color = ColorInput::Color(Color::Rgba(0, 0, 255, 1.0));
  style.border_color = Some(color.clone());
  assert_eq!(style.border_color, Some(color));
}

#[test]
fn test_inheritable_style_border_radius() {
  let mut style = InheritableStyle::default();
  let radius = SidesValue::SingleValue(LengthUnit::Px(8.0));
  style.border_radius = Some(radius);
  assert_eq!(style.border_radius, Some(radius));
}

#[test]
fn test_style_default_values() {
  let style = Style::default();
  assert_eq!(style.display, Display::Flex);
  assert_eq!(style.flex_direction, FlexDirection::Row);
  assert_eq!(style.flex_wrap, FlexWrap::NoWrap);
  assert_eq!(style.justify_content, None);
  assert_eq!(style.align_items, None);
  assert_eq!(style.position, Position::Relative);
  assert_eq!(style.object_fit, ObjectFit::Fill);
  assert_eq!(style.background_color, None);
  assert_eq!(style.box_shadow, None);
  assert_eq!(style.width, LengthUnit::Auto);
  assert_eq!(style.height, LengthUnit::Auto);
  assert_eq!(style.margin, SidesValue::SingleValue(LengthUnit::Px(0.0)));
  assert_eq!(style.padding, SidesValue::SingleValue(LengthUnit::Px(0.0)));
  assert_eq!(
    style.border_width,
    SidesValue::SingleValue(LengthUnit::Px(0.0))
  );
  assert_eq!(style.inheritable_style.border_radius, None);
  assert_eq!(style.inset, SidesValue::SingleValue(LengthUnit::Auto));
  assert_eq!(style.flex_grow, 0.0);
  assert_eq!(style.flex_shrink, 1.0);
  assert_eq!(style.flex_basis, LengthUnit::Auto);
  assert_eq!(style.gap, Gap::SingleValue(LengthUnit::Px(0.0)));
  assert_eq!(style.min_width, LengthUnit::Auto);
  assert_eq!(style.min_height, LengthUnit::Auto);
  assert_eq!(style.max_width, LengthUnit::Auto);
  assert_eq!(style.max_height, LengthUnit::Auto);
  assert_eq!(style.aspect_ratio, None);
  assert_eq!(style.grid_auto_columns, None);
  assert_eq!(style.grid_auto_rows, None);
  assert_eq!(style.grid_auto_flow, None);
  assert_eq!(style.grid_column, None);
  assert_eq!(style.grid_row, None);
  assert_eq!(style.grid_template_columns, None);
  assert_eq!(style.grid_template_rows, None);
}

#[test]
fn test_inheritable_style_default_values() {
  let style = InheritableStyle::default();
  assert_eq!(style.color, None);
  assert_eq!(style.font_size, None);
  assert_eq!(style.font_family, None);
  assert_eq!(style.line_height, None);
  assert_eq!(style.font_weight, None);
  assert_eq!(style.line_clamp, None);
  assert_eq!(style.text_align, None);
  assert_eq!(style.text_overflow, None);
  assert_eq!(style.border_color, None);
  assert_eq!(style.border_radius, None);
  assert_eq!(style.letter_spacing, None);
}
