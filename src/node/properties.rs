use ab_glyph::FontArc;
use serde::Deserialize;

use crate::{color::Color, context::Context, node::Node};

#[derive(Debug, Clone, Deserialize)]
pub struct RectProperties {
  pub color: Option<Color>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CircleProperties {
  pub color: Option<Color>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextProperties {
  pub content: String,
  #[serde(default = "TextProperties::default_font_size")]
  pub font_size: f32,
  pub font_family: Option<String>,
  #[serde(default = "TextProperties::default_line_height")]
  pub line_height: f32,
  pub color: Option<Color>,
}

impl TextProperties {
  pub fn default_line_height() -> f32 {
    1.2
  }

  pub fn default_font_size() -> f32 {
    16.0
  }

  pub fn font(&self, context: &Context) -> FontArc {
    let Some(font_family) = self.font_family.as_ref() else {
      return context.font_store.default_font();
    };

    context.font_store.get_font_or_default(font_family)
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageProperties {
  pub src: String,
  pub border_radius: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ContainerProperties {
  pub children: Vec<Node>,
}
