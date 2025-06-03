use ab_glyph::PxScale;
use image::{
  ImageError, RgbaImage,
  imageops::{FilterType, overlay, resize},
  load_from_memory,
};
use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use lru::LruCache;
use std::sync::Mutex;
use taffy::Layout;

use super::properties::{CircleProperties, ImageProperties, RectProperties, TextProperties};
use crate::context::Context;

pub type ImageFetchCache = Mutex<LruCache<String, ImageState>>;

#[derive(Debug)]
pub enum ImageState {
  Fetched(RgbaImage),
  NetworkError(reqwest::Error),
  DecodeError(ImageError),
}

pub fn draw_rect(props: &RectProperties, canvas: &mut RgbaImage, layout: Layout) {
  let color = props.color.unwrap_or_default();
  let rect = Rect::at(layout.location.x as i32, layout.location.y as i32)
    .of_size(layout.size.width as u32, layout.size.height as u32);
  draw_filled_rect_mut(canvas, rect, color.into());
}

pub fn draw_circle(props: &CircleProperties, canvas: &mut RgbaImage, layout: Layout) {
  let color = props.color.unwrap_or_default();
  let size = (layout.size.width.min(layout.size.height) / 2.0) as i32;
  draw_filled_circle_mut(
    canvas,
    (
      layout.location.x as i32 + size,
      layout.location.y as i32 + size,
    ),
    size,
    color.into(),
  );
}

pub fn draw_text(
  props: &TextProperties,
  context: &Context,
  canvas: &mut RgbaImage,
  layout: Layout,
) {
  let color = props.color.unwrap_or_default();
  let font_size = props.font_size.unwrap_or(16.0);
  let scale = PxScale::from(font_size);

  let font = if let Some(font_family) = props.font_family.as_ref() {
    context.font_store.get_font_or_default(font_family)
  } else {
    context.font_store.default_font()
  };

  draw_text_mut(
    canvas,
    color.into(),
    layout.location.x as i32,
    layout.location.y as i32,
    scale,
    &font,
    &props.content,
  );
}

pub fn draw_image(
  props: &ImageProperties,
  context: &Context,
  canvas: &mut RgbaImage,
  layout: Layout,
) {
  let mut lock = context.image_fetch_cache.lock().unwrap();
  let Some(ImageState::Fetched(image)) = lock.get(&props.src) else {
    return;
  };

  let should_resize = layout.content_size.width as u32 != image.width()
    || layout.content_size.height as u32 != image.height();

  if !should_resize {
    return overlay(
      canvas,
      image,
      (layout.location.x + layout.padding.left) as i64,
      (layout.location.y + layout.padding.top) as i64,
    );
  }

  let resized = resize(
    image,
    layout.content_size.width as u32,
    layout.content_size.height as u32,
    FilterType::Lanczos3,
  );

  overlay(
    canvas,
    &resized,
    (layout.location.x + layout.padding.left) as i64,
    (layout.location.y + layout.padding.top) as i64,
  );
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
