use parley::Alignment;
use serde::{Deserialize, Serialize};
use swash::text::WordBreakStrength;
use taffy::{Layout, Size, Style as TaffyStyle};
use ts_rs::TS;

use crate::{
  define_style, layout::{
    DEFAULT_LINE_HEIGHT_SCALER,
    style::{CssValue, properties::*},
  }, rendering::{BorderProperties, RenderContext}
};

/// Represents the resolved font style for a text node.
#[derive(Debug, Clone)]
pub struct ResolvedFontStyle {
  /// Font size in pixels for text rendering.
  pub font_size: f32,
  /// Line height as an absolute value in pixels.
  pub line_height: parley::LineHeight,
  /// Font weight for text rendering.
  pub font_weight: parley::FontWeight,
  /// Font slant style (normal, italic, oblique).
  pub font_style: parley::FontStyle,
  /// Controls variable font axis values for advanced typography.
  pub font_variation_settings: Option<FontVariationSettings>,
  /// Controls OpenType font features for advanced typography.
  pub font_feature_settings: Option<FontFeatureSettings>,
  /// Maximum number of lines for text before truncation.
  pub line_clamp: Option<u32>,
  /// Font family for text rendering.
  pub font_family: Option<FontFamily>,
  /// Letter spacing for text rendering in em units (relative to font size).
  pub letter_spacing: Option<f32>,
  /// Word spacing for text rendering in em units (relative to font size).
  pub word_spacing: Option<f32>,
  /// Text alignment within the element.
  pub text_align: Option<Alignment>,
  /// How text should be overflowed.
  pub text_overflow: TextOverflow,
  /// Text transform behavior (uppercase, lowercase, capitalize, none)
  pub text_transform: TextTransform,
  /// Text color for child text elements.
  pub color: Color,
  /// Text wrap behavior.
  pub overflow_wrap: parley::OverflowWrap,
  /// How text should be broken at word boundaries.
  pub word_break: WordBreakStrength,
}

define_style!(
  box_sizing: BoxSizing = BoxSizing::BorderBox.into(),
  color: Color = CssValue::Inherit,
  display: Display = Display::Flex.into(),
  width: LengthUnit = LengthUnit::Auto.into(),
  height: LengthUnit = LengthUnit::Auto.into(),
  max_width: LengthUnit = LengthUnit::Auto.into(),
  max_height: LengthUnit = LengthUnit::Auto.into(),
  min_width: LengthUnit = LengthUnit::Auto.into(),
  min_height: LengthUnit = LengthUnit::Auto.into(),
  aspect_ratio: Option<f32> = None.into(),
  padding: Sides<LengthUnit> = Sides([LengthUnit::zero(); 4]).into(),
  padding_top: Option<LengthUnit> = None.into(),
  padding_right: Option<LengthUnit> = None.into(),
  padding_bottom: Option<LengthUnit> = None.into(),
  padding_left: Option<LengthUnit> = None.into(),
  margin: Sides<LengthUnit> = Sides([LengthUnit::zero(); 4]).into(),
  margin_top: Option<LengthUnit> = None.into()
);

/// Main styling structure that contains all layout and visual properties.
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, optional_fields)]
pub struct Style {
  /// Display algorithm to use for the element.
  pub display: CssValue<Display>,
  /// Width of the element.
  pub width: CssValue<LengthUnit>,
  /// Height of the element.
  pub height: CssValue<LengthUnit>,
  /// Max width of the element.
  pub max_width: CssValue<LengthUnit>,
  /// Max height of the element.
  pub max_height: CssValue<LengthUnit>,
  /// Min width of the element.
  pub min_width: CssValue<LengthUnit>,
  /// Min height of the element.
  pub min_height: CssValue<LengthUnit>,
  /// Aspect ratio of the element (width/height).
  pub aspect_ratio: Option<f32>,
  /// Internal spacing around the element's content (top, right, bottom, left).
  pub padding: Sides<LengthUnit>,
  /// Longhand: top padding. Overrides `padding` top value.
  pub padding_top: Option<LengthUnit>,
  /// Longhand: right padding. Overrides `padding` right value.
  pub padding_right: Option<LengthUnit>,
  /// Longhand: bottom padding. Overrides `padding` bottom value.
  pub padding_bottom: Option<LengthUnit>,
  /// Longhand: left padding. Overrides `padding` left value.
  pub padding_left: Option<LengthUnit>,
  /// External spacing around the element (top, right, bottom, left).
  pub margin: Sides<LengthUnit>,
  /// Longhand: top margin. Overrides `margin` top value.
  pub margin_top: Option<LengthUnit>,
  /// Longhand: right margin. Overrides `margin` right value.
  pub margin_right: Option<LengthUnit>,
  /// Longhand: bottom margin. Overrides `margin` bottom value.
  pub margin_bottom: Option<LengthUnit>,
  /// Longhand: left margin. Overrides `margin` left value.
  pub margin_left: Option<LengthUnit>,
  /// Positioning offsets (top, right, bottom, left) from the element's normal position.
  pub inset: Sides<LengthUnit>,
  /// Longhand: top offset. Overrides `inset` top value.
  pub top: Option<LengthUnit>,
  /// Longhand: right offset. Overrides `inset` right value.
  pub right: Option<LengthUnit>,
  /// Longhand: bottom offset. Overrides `inset` bottom value.
  pub bottom: Option<LengthUnit>,
  /// Longhand: left offset. Overrides `inset` left value.
  pub left: Option<LengthUnit>,
  /// Direction of flex layout (row or column).
  pub flex_direction: FlexDirection,
  /// How a single grid item is aligned along the inline (row) axis, overriding the container's justify-items value.
  pub justify_self: Option<AlignItems>,
  /// How items are aligned along the main axis.
  pub justify_content: Option<JustifyContent>,
  /// How lines are aligned within the flex container when there's extra space in the cross axis.
  pub align_content: Option<JustifyContent>,
  /// How grid items are aligned along the inline (row) axis within their grid areas.
  pub justify_items: Option<AlignItems>,
  /// How items are aligned along the cross axis.
  pub align_items: Option<AlignItems>,
  /// How a single item is aligned along the cross axis, overriding the container's align-items value.
  pub align_self: Option<AlignItems>,
  /// How flex items should wrap.
  pub flex_wrap: FlexWrap,
  /// The initial main size of the flex item before growing or shrinking.
  pub flex_basis: LengthUnit,
  /// Positioning method (relative, absolute, etc.).
  pub position: CssValue<Position>,
  /// Transform for the element.
  pub transform: Option<Transforms>,
  /// Transform origin for the element.
  pub transform_origin: Option<BackgroundPosition>,
  /// Mask image for the element.
  pub mask_image: Option<BackgroundImages>,
  /// Mask size for the element.
  pub mask_size: Option<BackgroundSizes>,
  /// Mask position for the element.
  pub mask_position: Option<BackgroundPositions>,
  /// Mask repeat for the element.
  pub mask_repeat: Option<BackgroundRepeats>,
  /// Spacing between rows and columns in flex or grid layouts.
  pub gap: Gap,
  /// How much the flex item should grow relative to other flex items when positive free space is distributed.
  pub flex_grow: f32,
  /// How much the flex item should shrink relative to other flex items when negative free space is distributed.
  pub flex_shrink: f32,
  /// Shorthand border radius (top-left, top-right, bottom-right, bottom-left).
  pub border_radius: Option<Sides<LengthUnit>>,
  /// Longhand: top-left border radius. Overrides `border_radius` top-left value.
  pub border_top_left_radius: Option<LengthUnit>,
  /// Longhand: top-right border radius. Overrides `border_radius` top-right value.
  pub border_top_right_radius: Option<LengthUnit>,
  /// Longhand: bottom-right border radius. Overrides `border_radius` bottom-right value.
  pub border_bottom_right_radius: Option<LengthUnit>,
  /// Longhand: bottom-left border radius. Overrides `border_radius` bottom-left value.
  pub border_bottom_left_radius: Option<LengthUnit>,
  /// Width of the element's border on each side (top, right, bottom, left).
  pub border_width: Sides<LengthUnit>,
  /// Longhand: top border width. Overrides `border_width` top value.
  pub border_top_width: Option<LengthUnit>,
  /// Longhand: right border width. Overrides `border_width` right value.
  pub border_right_width: Option<LengthUnit>,
  /// Longhand: bottom border width. Overrides `border_width` bottom value.
  pub border_bottom_width: Option<LengthUnit>,
  /// Longhand: left border width. Overrides `border_width` left value.
  pub border_left_width: Option<LengthUnit>,
  /// How images should be fitted within their container.
  pub object_fit: ObjectFit,
  /// Background image(s): linear or radial gradients.
  pub background_image: Option<BackgroundImages>,
  /// Background positions per layer.
  pub background_position: Option<BackgroundPositions>,
  /// Background sizes per layer.
  pub background_size: Option<BackgroundSizes>,
  /// Background repeat per layer.
  pub background_repeat: Option<BackgroundRepeats>,
  /// Background color for the element.
  pub background_color: Option<Color>,
  /// Box shadow effect for the element.
  pub box_shadow: Option<BoxShadows>,
  /// Controls the size of implicitly-created grid columns.
  pub grid_auto_columns: Option<GridTrackSizes>,
  /// Controls the size of implicitly-created grid rows.
  pub grid_auto_rows: Option<GridTrackSizes>,
  /// Controls how auto-placed items are inserted in the grid.
  pub grid_auto_flow: Option<GridAutoFlow>,
  /// Specifies a grid item's size and location within the grid column.
  pub grid_column: Option<GridLine>,
  /// Specifies a grid item's size and location within the grid row.
  pub grid_row: Option<GridLine>,
  /// Defines the line names and track sizing functions of the grid columns.
  pub grid_template_columns: Option<GridTemplateComponents>,
  /// Defines the line names and track sizing functions of the grid rows.
  pub grid_template_rows: Option<GridTemplateComponents>,
  /// Defines named grid areas specified via `grid-template-areas`.
  pub grid_template_areas: Option<GridTemplateAreas>,

  // Inheritable style properties that cascade to child elements
  /// How the width and height of an element are calculated.
  pub box_sizing: CssValue<BoxSizing>,
  /// How text should be overflowed.
  pub text_overflow: CssValue<TextOverflow>,
  /// Controls text case transformation when rendering.
  pub text_transform: CssValue<TextTransform>,
  /// Font slant style (normal, italic, oblique).
  pub font_style: CssValue<FontStyle>,
  /// Color of the element's border.
  pub border_color: CssValue<Color>,
  /// Text color for child text elements.
  pub color: CssValue<Color>,
  /// Font size for text rendering.
  pub font_size: CssValue<LengthUnit>,
  /// Font family name for text rendering.
  pub font_family: CssValue<FontFamily>,
  /// Line height for text spacing, number is em.
  pub line_height: CssValue<LineHeight>,
  /// Font weight for text rendering.
  pub font_weight: CssValue<FontWeight>,
  /// Controls variable font axis values via CSS font-variation-settings property.
  pub font_variation_settings: CssValue<FontVariationSettings>,
  /// Controls OpenType font features via CSS font-feature-settings property.
  pub font_feature_settings: CssValue<FontFeatureSettings>,
  /// Maximum number of lines for text before truncation.
  pub line_clamp: CssValue<u32>,
  /// Text alignment within the element.
  pub text_align: CssValue<TextAlign>,
  /// Additional spacing between characters in text.
  pub letter_spacing: CssValue<LengthUnit>,
  /// Additional spacing between words in text.
  pub word_spacing: CssValue<LengthUnit>,
  /// Controls how images are scaled when rendered.
  pub image_rendering: CssValue<ImageScalingAlgorithm>,
  /// How text should be overflowed.
  pub overflow_wrap: CssValue<OverflowWrap>,
  /// How text should be broken at word boundaries.
  pub word_break: CssValue<WordBreak>,
}

impl Default for Style {
  fn default() -> Self {
    Self {
      display: CssValue::Value(Display::Flex),
      margin: Sides([LengthUnit::zero(); 4]),
      margin_top: None,
      margin_right: None,
      margin_bottom: None,
      margin_left: None,
      padding: Sides([LengthUnit::zero(); 4]),
      padding_top: None,
      padding_right: None,
      padding_bottom: None,
      padding_left: None,
      width: CssValue::Unset,
      height: CssValue::Unset,
      max_width: CssValue::Unset,
      max_height: CssValue::Unset,
      min_width: CssValue::Unset,
      min_height: CssValue::Unset,
      aspect_ratio: None,
      inset: Default::default(),
      top: None,
      right: None,
      bottom: None,
      left: None,
      flex_direction: Default::default(),
      justify_content: Default::default(),
      align_content: Default::default(),
      justify_self: Default::default(),
      justify_items: Default::default(),
      align_items: Default::default(),
      align_self: Default::default(),
      position: CssValue::Value(Position::Relative),
      gap: Default::default(),
      flex_grow: 0.0,
      flex_shrink: 1.0,
      flex_basis: Default::default(),
      flex_wrap: FlexWrap::NoWrap,
      border_width: Sides([LengthUnit::zero(); 4]),
      border_top_width: None,
      border_right_width: None,
      border_bottom_width: None,
      border_left_width: None,
      border_radius: None,
      border_top_left_radius: None,
      border_top_right_radius: None,
      border_bottom_right_radius: None,
      border_bottom_left_radius: None,
      object_fit: Default::default(),
      box_shadow: Default::default(),
      transform: None,
      transform_origin: None,
      mask_image: None,
      mask_size: None,
      mask_position: None,
      mask_repeat: None,
      background_color: None,
      background_image: None,
      background_position: None,
      background_size: None,
      background_repeat: None,
      grid_auto_columns: None,
      grid_auto_rows: None,
      grid_auto_flow: None,
      grid_column: None,
      grid_row: None,
      grid_template_columns: None,
      grid_template_rows: None,
      grid_template_areas: None,

      // Inheritable style properties
      box_sizing: CssValue::Unset,
      text_overflow: CssValue::Unset,
      text_transform: CssValue::Unset,
      font_style: CssValue::Unset,
      border_color: CssValue::Unset,
      color: CssValue::Unset,
      font_size: CssValue::Unset,
      font_family: CssValue::Unset,
      line_height: CssValue::Unset,
      font_weight: CssValue::Unset,
      font_variation_settings: CssValue::Unset,
      font_feature_settings: CssValue::Unset,
      line_clamp: CssValue::Unset,
      text_align: CssValue::Unset,
      letter_spacing: CssValue::Unset,
      word_spacing: CssValue::Unset,
      image_rendering: CssValue::Unset,
      overflow_wrap: CssValue::Unset,
      word_break: CssValue::Unset,
    }
  }
}

impl Style {
  #[inline]
  fn convert_template_components(
    components: &Option<GridTemplateComponents>,
    context: &RenderContext,
  ) -> (Vec<taffy::GridTemplateComponent<String>>, Vec<Vec<String>>) {
    let mut track_components: Vec<taffy::GridTemplateComponent<String>> = Vec::new();
    let mut line_name_sets: Vec<Vec<String>> = Vec::new();
    let mut pending_line_names: Vec<String> = Vec::new();

    if let Some(list) = components {
      for comp in list.0.iter() {
        match comp {
          GridTemplateComponent::LineNames(names) => {
            if !names.is_empty() {
              pending_line_names.extend_from_slice(&names[..]);
            }
          }
          GridTemplateComponent::Single(track_size) => {
            // Push names for the line preceding this track
            line_name_sets.push(std::mem::take(&mut pending_line_names));
            // Push the track component
            track_components.push(taffy::GridTemplateComponent::Single(
              track_size.to_min_max(context),
            ));
          }
          GridTemplateComponent::Repeat(repetition, tracks) => {
            // Push names for the line preceding this repeat fragment
            line_name_sets.push(std::mem::take(&mut pending_line_names));

            // Build repetition
            let track_sizes: Vec<taffy::TrackSizingFunction> =
              tracks.iter().map(|t| t.size.to_min_max(context)).collect();

            // Build inner line names: one per line inside the repeat, including a trailing set
            let mut inner_line_names: Vec<Vec<String>> =
              tracks.iter().map(|t| t.names.clone()).collect();
            if let Some(last) = tracks.last() {
              if let Some(end) = &last.end_names {
                inner_line_names.push(end.clone());
              } else {
                inner_line_names.push(Vec::new());
              }
            } else {
              inner_line_names.push(Vec::new());
            }

            track_components.push(taffy::GridTemplateComponent::Repeat(
              taffy::GridTemplateRepetition {
                count: (*repetition).into(),
                tracks: track_sizes,
                line_names: inner_line_names,
              },
            ));
          }
        }
      }
    }

    // Trailing names after the last track
    line_name_sets.push(pending_line_names);

    (track_components, line_name_sets)
  }

  #[inline]
  fn resolve_rect_with_longhands(
    base: Sides<LengthUnit>,
    top: Option<LengthUnit>,
    right: Option<LengthUnit>,
    bottom: Option<LengthUnit>,
    left: Option<LengthUnit>,
  ) -> taffy::Rect<LengthUnit> {
    let mut values = base.0;
    if let Some(v) = top {
      values[0] = v;
    }
    if let Some(v) = right {
      values[1] = v;
    }
    if let Some(v) = bottom {
      values[2] = v;
    }
    if let Some(v) = left {
      values[3] = v;
    }
    taffy::Rect {
      top: values[0],
      right: values[1],
      bottom: values[2],
      left: values[3],
    }
  }

  #[inline]
  fn resolved_padding(&self) -> taffy::Rect<LengthUnit> {
    Self::resolve_rect_with_longhands(
      self.padding,
      self.padding_top,
      self.padding_right,
      self.padding_bottom,
      self.padding_left,
    )
  }

  #[inline]
  fn resolved_margin(&self) -> taffy::Rect<LengthUnit> {
    Self::resolve_rect_with_longhands(
      self.margin,
      self.margin_top,
      self.margin_right,
      self.margin_bottom,
      self.margin_left,
    )
  }

  #[inline]
  fn resolved_inset(&self) -> taffy::Rect<LengthUnit> {
    Self::resolve_rect_with_longhands(self.inset, self.top, self.right, self.bottom, self.left)
  }

  #[inline]
  fn resolved_border_width(&self) -> taffy::Rect<LengthUnit> {
    Self::resolve_rect_with_longhands(
      self.border_width,
      self.border_top_width,
      self.border_right_width,
      self.border_bottom_width,
      self.border_left_width,
    )
  }

  #[inline]
  fn resolved_border_radius(&self) -> taffy::Rect<LengthUnit> {
    Self::resolve_rect_with_longhands(
      self.border_radius.unwrap_or_default(),
      self.border_top_left_radius,
      self.border_top_right_radius,
      self.border_bottom_right_radius,
      self.border_bottom_left_radius,
    )
  }

  /// Creates `BorderProperties` (including resolved corner radii) from the style's border radius properties.
  #[inline]
  pub fn create_border_radius(&self, layout: &Layout, context: &RenderContext) -> BorderProperties {
    let rect = self.resolved_border_radius();

    BorderProperties::from_resolved(context, layout, rect, self)
  }

  /// Converts this style to a Taffy-compatible style for layout calculations.
  pub fn resolve_to_taffy_style(&self, context: &RenderContext) -> TaffyStyle {
    // Helper function to resolve CssValue to concrete value with defaults
    fn resolve_css_value_with_default<T: Default + Clone>(
      css_value: &CssValue<T>,
      default_value: T,
    ) -> T {
      match css_value {
        CssValue::Value(v) => v.clone(),
        CssValue::Inherit => default_value, // For now, fall back to default; inheritance would be handled at a higher level
        CssValue::Unset => default_value,
      }
    }

    // Convert grid templates and associated line names
    let (grid_template_columns, grid_template_column_names) =
      Self::convert_template_components(&self.grid_template_columns, context);
    let (grid_template_rows, grid_template_row_names) =
      Self::convert_template_components(&self.grid_template_rows, context);

    TaffyStyle {
      box_sizing: self
        .box_sizing
        .as_value()
        .cloned()
        .unwrap_or_default()
        .into(),
      size: Size {
        width: resolve_css_value_with_default(&self.width, LengthUnit::Auto)
          .resolve_to_dimension(context),
        height: resolve_css_value_with_default(&self.height, LengthUnit::Auto)
          .resolve_to_dimension(context),
      },
      border: resolve_length_unit_rect_to_length_percentage(context, self.resolved_border_width()),
      padding: resolve_length_unit_rect_to_length_percentage(context, self.resolved_padding()),
      inset: resolve_length_unit_rect_to_length_percentage_auto(context, self.resolved_inset()),
      margin: resolve_length_unit_rect_to_length_percentage_auto(context, self.resolved_margin()),
      display: resolve_css_value_with_default(&self.display, Display::Flex).into(),
      flex_direction: self.flex_direction.into(),
      position: resolve_css_value_with_default(&self.position, Position::Relative).into(),
      justify_content: self.justify_content.map(Into::into),
      align_content: self.align_content.map(Into::into),
      justify_items: self.justify_items.map(Into::into),
      flex_grow: self.flex_grow,
      align_items: self.align_items.map(Into::into),
      gap: self.gap.resolve_to_size(context),
      flex_basis: self.flex_basis.resolve_to_dimension(context),
      flex_shrink: self.flex_shrink,
      flex_wrap: self.flex_wrap.into(),
      min_size: Size {
        width: resolve_css_value_with_default(&self.min_width, LengthUnit::Auto)
          .resolve_to_dimension(context),
        height: resolve_css_value_with_default(&self.min_height, LengthUnit::Auto)
          .resolve_to_dimension(context),
      },
      max_size: Size {
        width: resolve_css_value_with_default(&self.max_width, LengthUnit::Auto)
          .resolve_to_dimension(context),
        height: resolve_css_value_with_default(&self.max_height, LengthUnit::Auto)
          .resolve_to_dimension(context),
      },
      grid_auto_columns: self.grid_auto_columns.as_ref().map_or_else(Vec::new, |v| {
        v.0.iter().map(|s| s.to_min_max(context)).collect()
      }),
      grid_auto_rows: self.grid_auto_rows.as_ref().map_or_else(Vec::new, |v| {
        v.0.iter().map(|s| s.to_min_max(context)).collect()
      }),
      grid_auto_flow: self.grid_auto_flow.unwrap_or_default().into(),
      grid_column: self
        .grid_column
        .as_ref()
        .map_or_else(Default::default, |line| line.clone().into()),
      grid_row: self
        .grid_row
        .as_ref()
        .map_or_else(Default::default, |line| line.clone().into()),
      grid_template_columns,
      grid_template_rows,
      grid_template_column_names,
      grid_template_row_names,
      grid_template_areas: self
        .grid_template_areas
        .as_ref()
        .cloned()
        .unwrap_or_default()
        .into(),
      aspect_ratio: self.aspect_ratio,
      align_self: self.align_self.map(Into::into),
      justify_self: self.justify_self.map(Into::into),
      ..Default::default()
    }
  }

  /// Resolves inheritable style properties to concrete values for text rendering.
  pub fn resolve_to_font_style(&self, context: &RenderContext) -> ResolvedFontStyle {
    // Helper function to resolve a CssValue to a concrete value
    fn resolve_css_value<T: Clone>(
      css_value: &CssValue<T>,
      inherited_value: Option<&T>,
      initial_value: T,
    ) -> T {
      match css_value {
        CssValue::Value(v) => v.clone(),
        CssValue::Inherit => inherited_value.cloned().unwrap_or(initial_value),
        CssValue::Unset => initial_value,
      }
    }

    let font_size = match &self.font_size {
      CssValue::Value(f) => f.resolve_to_px(context, context.parent_font_size),
      CssValue::Inherit => context.parent_font_size,
      CssValue::Unset => context.parent_font_size, // Default font size
    };

    let line_height = match &self.line_height {
      CssValue::Value(line_height) => line_height.into_parley(context),
      CssValue::Inherit | CssValue::Unset => {
        parley::LineHeight::FontSizeRelative(DEFAULT_LINE_HEIGHT_SCALER)
      }
    };

    ResolvedFontStyle {
      color: resolve_css_value(&self.color, None, Color::black()),
      font_size,
      line_height,
      font_weight: resolve_css_value(&self.font_weight, None, FontWeight::default()).into(),
      font_style: resolve_css_value(&self.font_style, None, FontStyle::default()).into(),
      font_variation_settings: self.font_variation_settings.as_value().cloned(),
      font_feature_settings: self.font_feature_settings.as_value().cloned(),
      line_clamp: self.line_clamp.as_value().copied(),
      font_family: self.font_family.as_value().cloned(),
      letter_spacing: self
        .letter_spacing
        .as_value()
        .map(|spacing| spacing.resolve_to_px(context, font_size) / font_size),
      word_spacing: self
        .word_spacing
        .as_value()
        .map(|spacing| spacing.resolve_to_px(context, font_size) / font_size),
      text_align: self.text_align.as_value().map(|v| (*v).into()),
      text_overflow: resolve_css_value(&self.text_overflow, None, TextOverflow::Clip),
      text_transform: resolve_css_value(&self.text_transform, None, TextTransform::default()),
      overflow_wrap: resolve_css_value(&self.overflow_wrap, None, OverflowWrap::default()).into(),
      word_break: resolve_css_value(&self.word_break, None, WordBreak::default()).into(),
    }
  }
}
