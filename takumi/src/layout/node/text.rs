//! Text node implementation for the takumi layout system.
//!
//! This module contains the TextNode struct which is used to render
//! text content with configurable font properties and styling.

use serde::{Deserialize, Serialize};
use taffy::{AvailableSpace, Layout, Size};

use crate::{
  GlobalContext,
  layout::{
    node::Node,
    style::{ResolvedFontStyle, Style},
  },
  rendering::{
    Canvas, MaxHeight, RenderContext, apply_text_transform, create_text_layout, draw_text,
  },
};

/// A node that renders text content.
///
/// Text nodes display text with configurable font properties,
/// alignment, and styling options.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextNode {
  /// The styling properties for this text node
  #[serde(default)]
  pub style: Style,
  /// The text content to be rendered
  pub text: String,
}

impl<Nodes: Node<Nodes>> Node<Nodes> for TextNode {
  fn get_style(&self) -> &Style {
    &self.style
  }

  fn get_style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

  fn draw_content(&self, context: &RenderContext, canvas: &Canvas, layout: Layout) {
    draw_text(&self.text, &self.style, context, canvas, layout);
  }

  fn measure(
    &self,
    context: &RenderContext,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    measure_text(
      context.global,
      &self.text,
      &self.style.resolve_to_font_style(context),
      known_dimensions,
      available_space,
    )
  }

  fn has_draw_content(&self) -> bool {
    true
  }
}

/// Measures the size of text based on font style and available space.
///
/// This function handles text wrapping, line height, and respects both explicit
/// dimensions and available space constraints.
pub fn measure_text(
  global: &GlobalContext,
  text: &str,
  style: &ResolvedFontStyle,
  known_dimensions: Size<Option<f32>>,
  available_space: Size<AvailableSpace>,
) -> Size<f32> {
  if text.trim().is_empty()
    || known_dimensions.width == Some(0.0)
    || known_dimensions.height == Some(0.0)
  {
    return Size {
      width: 0.0,
      height: 0.0,
    };
  }

  let width_constraint = known_dimensions.width.or(match available_space.width {
    AvailableSpace::MinContent => Some(0.0),
    AvailableSpace::MaxContent => None,
    AvailableSpace::Definite(width) => Some(width),
  });

  let height_constraint = known_dimensions.height.or(match available_space.height {
    AvailableSpace::MinContent => Some(0.0),
    AvailableSpace::MaxContent => None,
    AvailableSpace::Definite(height) => Some(height),
  });

  let height_constraint_with_max_lines = match (style.line_clamp, height_constraint) {
    (Some(max_lines), Some(height)) => Some(MaxHeight::Both(height, max_lines)),
    (Some(max_lines), None) => Some(MaxHeight::Lines(max_lines)),
    (None, Some(height)) => Some(MaxHeight::Absolute(height)),
    (None, None) => None,
  };

  let text = apply_text_transform(text, style.text_transform);

  let buffer = create_text_layout(
    &text,
    style,
    global,
    width_constraint.unwrap_or(f32::MAX),
    height_constraint_with_max_lines,
  );

  let (max_run_width, total_height) =
    buffer
      .lines()
      .fold((0.0, 0.0), |(max_run_width, total_height), line| {
        let metrics = line.metrics();
        (
          metrics.advance.max(max_run_width),
          total_height + metrics.line_height,
        )
      });

  taffy::Size {
    width: max_run_width
      .ceil()
      .min(width_constraint.unwrap_or(f32::MAX)),
    height: total_height.ceil(),
  }
}
