//! Text node implementation for the takumi layout system.
//!
//! This module contains the TextNode struct which is used to render
//! text content with configurable font properties and styling.

use serde::Deserialize;
use taffy::{AvailableSpace, Layout, Size};

use crate::{
  core::RenderContext,
  layout::{measure_text, trait_node::Node},
  rendering::{FastBlendImage, draw_text},
  style::Style,
};

/// A node that renders text content.
///
/// Text nodes display text with configurable font properties,
/// alignment, and styling options.
#[derive(Debug, Clone, Deserialize)]
pub struct TextNode {
  /// The styling properties for this text node
  #[serde(default, flatten)]
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

  fn draw_content(&self, context: &RenderContext, canvas: &mut FastBlendImage, layout: Layout) {
    draw_text(
      &self.text,
      &self.style.resolve_to_font_style(context),
      context,
      canvas,
      layout,
    );
  }

  fn measure(
    &self,
    context: &RenderContext,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    measure_text(
      context,
      &self.text,
      &self.style.resolve_to_font_style(context),
      known_dimensions,
      available_space,
    )
  }
}
