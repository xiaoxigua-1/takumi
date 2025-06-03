use serde::Deserialize;

use crate::{color::Color, node::Node};

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
  pub font_size: Option<f32>,
  pub font_family: Option<String>,
  pub color: Option<Color>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageProperties {
  pub src: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ContainerProperties {
  pub children: Vec<Node>,
}
