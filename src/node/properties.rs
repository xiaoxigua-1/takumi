use cosmic_text::Weight;
use image::load_from_memory;
use serde::Deserialize;

use crate::{
  color::Color,
  context::Context,
  node::{Node, draw::ImageState},
};

#[derive(Debug, Clone, Deserialize)]
pub struct RectProperties {
  pub color: Option<Color>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CircleProperties {
  pub color: Option<Color>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct FontWeight(u16);

#[derive(Debug, Clone, Deserialize)]
pub struct TextProperties {
  pub content: String,
  #[serde(default = "TextProperties::default_font_size")]
  pub font_size: f32,
  pub font_family: Option<String>,
  #[serde(default = "TextProperties::default_line_height")]
  pub line_height: f32,
  #[serde(default)]
  pub font_weight: FontWeight,
  #[serde(default)]
  pub color: Color,
}

impl Default for FontWeight {
  fn default() -> Self {
    FontWeight(Weight::NORMAL.0)
  }
}

impl From<FontWeight> for Weight {
  fn from(weight: FontWeight) -> Self {
    Weight(weight.0)
  }
}

impl TextProperties {
  pub fn default_line_height() -> f32 {
    1.2
  }

  pub fn default_font_size() -> f32 {
    16.0
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageProperties {
  pub src: String,
  pub border_radius: Option<f32>,
}

impl ImageProperties {
  pub async fn fetch_and_store(&self, context: &Context) {
    let is_cached = {
      let mut lock = context.image_fetch_cache.lock().unwrap();
      matches!(
        lock.get(&self.src),
        Some(ImageState::Fetched(_) | ImageState::NetworkError(_))
      )
    };

    if is_cached {
      return;
    }

    let state = self.fetch_state().await;

    let mut cache = context.image_fetch_cache.lock().unwrap();
    cache.put(self.src.clone(), state);
  }

  async fn fetch_state(&self) -> ImageState {
    let response = reqwest::get(&self.src).await;

    if let Err(e) = response {
      return ImageState::NetworkError(e);
    }

    let response = response.unwrap();
    let bytes = response.bytes().await;

    if let Err(e) = bytes {
      return ImageState::NetworkError(e);
    }

    let image = load_from_memory(&bytes.unwrap());

    if let Err(e) = image {
      return ImageState::DecodeError(e);
    }

    ImageState::Fetched(image.unwrap().to_rgba8())
  }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ContainerProperties {
  pub children: Vec<Node>,
}
