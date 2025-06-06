use cosmic_text::{Attrs, Buffer, Metrics, Shaping};
use image::{
  ImageError, Rgba, RgbaImage,
  imageops::{FilterType, overlay, resize},
};
use imageproc::drawing::{Blend, draw_filled_circle_mut};
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

pub fn draw_rect(props: &RectProperties, canvas: &mut Blend<RgbaImage>, layout: Layout) {
  let content_box = layout.content_box_size();
  let x = layout.content_box_x();
  let y = layout.content_box_y();

  let color = props.color.unwrap_or_default();
  let rect =
    Rect::at(x as i32, y as i32).of_size(content_box.width as u32, content_box.height as u32);

  draw_filled_rect_mut(canvas, rect, color.into());
}

pub fn draw_circle(props: &CircleProperties, canvas: &mut Blend<RgbaImage>, layout: Layout) {
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
  canvas: &mut Blend<RgbaImage>,
  layout: Layout,
) {
  let content_box = layout.content_box_size();

  let start_x = layout.content_box_x();
  let start_y = layout.content_box_y() + props.font_size * ((props.line_height - 1.0) / 2.0);

  let mut font_system = context.font_system.lock().unwrap();

  let metrics = Metrics::relative(props.font_size, props.line_height);
  let mut buffer = Buffer::new(&mut font_system, metrics);

  let attrs = Attrs::new().weight(props.font_weight.into());

  buffer.set_text(&mut font_system, &props.content, &attrs, Shaping::Advanced);
  buffer.set_size(
    &mut font_system,
    Some(content_box.width),
    Some(content_box.height),
  );

  buffer.shape_until_scroll(&mut font_system, true);

  let mut font_cache = context.font_cache.lock().unwrap();

  buffer.draw(
    &mut font_system,
    &mut font_cache,
    props.color.into(),
    |x, y, w, h, color| {
      if color.a() == 0
        || x < 0
        || x >= content_box.width as i32
        || y < 0
        || y >= content_box.height as i32
        || w != 1
        || h != 1
      {
        // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
        return;
      }

      draw_filled_rect_mut(
        canvas,
        Rect::at(start_x as i32 + x, start_y as i32 + y).of_size(w, h),
        Rgba(color.as_rgba()),
      );
    },
  );
}

pub fn draw_image(
  props: &ImageProperties,
  context: &Context,
  canvas: &mut Blend<RgbaImage>,
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
    return overlay(&mut canvas.0, image, x as i64, y as i64);
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

  overlay(&mut canvas.0, &resized, x as i64, y as i64);
}
