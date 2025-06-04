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
use crate::{border_radius::apply_border_radius_antialiased, context::Context};

pub type ImageFetchCache = Mutex<LruCache<String, ImageState>>;

#[derive(Debug)]
pub enum ImageState {
  Fetched(RgbaImage),
  NetworkError(reqwest::Error),
  DecodeError(ImageError),
}

pub fn draw_rect(props: &RectProperties, canvas: &mut RgbaImage, layout: Layout) {
  let content_box = layout.content_box_size();
  let x = layout.content_box_x();
  let y = layout.content_box_y();

  let color = props.color.unwrap_or_default();
  let rect =
    Rect::at(x as i32, y as i32).of_size(content_box.width as u32, content_box.height as u32);

  draw_filled_rect_mut(canvas, rect, color.into());
}

pub fn draw_circle(props: &CircleProperties, canvas: &mut RgbaImage, layout: Layout) {
  let content_box = layout.content_box_size();
  let x = layout.content_box_x();
  let y = layout.content_box_y();

  let color = props.color.unwrap_or_default();
  let size = content_box.width.min(content_box.height) / 2.0;

  draw_filled_circle_mut(
    canvas,
    ((x + size) as i32, (y + size) as i32),
    size as i32,
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
  let scale = PxScale::from(props.font_size);

  let x = layout.content_box_x();
  let y = layout.content_box_y();

  let font = props.font(context);

  draw_text_mut(
    canvas,
    color.into(),
    x as i32,
    y as i32,
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

  let content_box = layout.content_box_size();
  let x = layout.content_box_x();
  let y = layout.content_box_y();

  let should_resize =
    content_box.width as u32 != image.width() || content_box.height as u32 != image.height();

  if !should_resize && props.border_radius.is_none() {
    return overlay(canvas, image, x as i64, y as i64);
  }

  let mut resized = resize(
    image,
    content_box.width as u32,
    content_box.height as u32,
    FilterType::Lanczos3,
  );

  if let Some(border_radius) = props.border_radius {
    apply_border_radius_antialiased(&mut resized, border_radius);
  }

  overlay(canvas, &resized, x as i64, y as i64);
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
