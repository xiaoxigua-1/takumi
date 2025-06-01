use std::{num::NonZeroUsize, sync::LazyLock};

use ab_glyph::PxScale;
use image::{
  RgbaImage,
  imageops::{FilterType, overlay, resize},
};
use imageproc::{
  drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_text_mut},
  rect::Rect,
};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
  draw::{border_radius::apply_border_radius_optimized, rgb::parse_rgb},
  font::FONT,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Canvas {
  pub width: u32,
  pub height: u32,
  pub bg_color: u32,
  pub paths: Vec<Path>,
  pub border_radius: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Path {
  #[serde(rename = "text")]
  Text {
    text: String,
    x: i32,
    y: i32,
    font_size: f32,
    color: u32,
  },
  #[serde(rename = "image")]
  Image {
    url: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    border_radius: Option<u32>,
  },
  #[serde(rename = "rect")]
  Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: u32,
  },
  #[serde(rename = "circle")]
  Circle {
    x: i32,
    y: i32,
    radius: i32,
    color: u32,
  },
}

static IMAGE_CACHE: LazyLock<Mutex<LruCache<String, RgbaImage>>> =
  LazyLock::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(10).unwrap())));

async fn get_image(url: String) -> RgbaImage {
  let mut cache = IMAGE_CACHE.lock().await;

  if let Some(image) = cache.get(&url) {
    return image.clone();
  }

  drop(cache);

  let response = reqwest::get(&url).await.unwrap();
  let bytes = response.bytes().await.unwrap();
  let downloaded: RgbaImage = image::load_from_memory(&bytes).unwrap().into();

  let mut cache = IMAGE_CACHE.lock().await;

  cache.put(url, downloaded.clone());

  downloaded
}

pub async fn draw_path(image: &mut RgbaImage, path: Path) {
  match path {
    Path::Text {
      text,
      x,
      y,
      font_size,
      color,
    } => {
      let text_color = parse_rgb(color);

      let font_size = PxScale::from(font_size);

      draw_text_mut(image, text_color, x, y, font_size, &*FONT, &text);
    }
    Path::Image {
      url,
      x,
      y,
      width,
      height,
      border_radius,
    } => {
      let mut downloaded = get_image(url).await;

      if downloaded.width() != width || downloaded.height() != height {
        downloaded = resize(&downloaded, width, height, FilterType::Lanczos3);
      }

      if let Some(border_radius) = border_radius {
        let trimmed_radius = border_radius.min(width.max(height) / 2);

        apply_border_radius_optimized(&mut downloaded, trimmed_radius);
      }

      overlay(image, &downloaded, x.into(), y.into());
    }
    Path::Rect {
      x,
      y,
      width,
      height,
      color,
    } => {
      draw_filled_rect_mut(
        image,
        Rect::at(x, y).of_size(width, height),
        parse_rgb(color),
      );
    }
    Path::Circle {
      x,
      y,
      radius,
      color,
    } => {
      draw_filled_circle_mut(image, (x, y), radius, parse_rgb(color));
    }
  }
}

// Example usage:
#[cfg(test)]
mod tests {
  use serde_json::from_str;

  use super::*;

  #[test]
  fn test_deserialization() {
    let json = r#"
        {
            "width": 1920,
            "height": 1080,
            "bg_color": 0,
            "paths": [
                {
                    "type": "text",
                    "text": "Hello, World!",
                    "x": 100,
                    "y": 100,
                    "font_size": 64,
                    "color": 0
                },
                {
                    "type": "image",
                    "url": "https://example.com/image.png",
                    "x": 100,
                    "y": 100,
                    "width": 100,
                    "height": 100
                },
                {
                    "type": "rect",
                    "x": 100,
                    "y": 100,
                    "width": 100,
                    "height": 100,
                    "color": 0
                }
            ]
        }
        "#;

    let canvas: Canvas = from_str(json).unwrap();
    println!("{:#?}", canvas);
  }
}
