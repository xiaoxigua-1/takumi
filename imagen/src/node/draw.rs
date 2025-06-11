use cosmic_text::{Attrs, Buffer, Family, Metrics, Shaping};
use image::{
  ImageError, Rgba, RgbaImage,
  imageops::{FilterType, overlay, resize},
};
use imageproc::drawing::Blend;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use taffy::Layout;

use crate::{
  border_radius::apply_border_radius_antialiased,
  context::FontContext,
  node::style::{FontStyle, Style},
};

/// Represents the state of an image in the rendering system.
///
/// This enum tracks whether an image has been successfully loaded and decoded,
/// or if there was an error during the process.
#[derive(Debug)]
pub enum ImageState {
  /// The image has been successfully loaded and decoded
  Fetched(RgbaImage),
  /// An error occurred while fetching the image from the network
  NetworkError,
  /// An error occurred while decoding the image data
  DecodeError(ImageError),
}

/// Draws text on the canvas with the specified font style and layout.
///
/// # Arguments
/// * `text` - The text to draw
/// * `font_style` - The font styling to apply
/// * `context` - The font context containing font system and cache
/// * `canvas` - The canvas to draw on
/// * `layout` - The layout information for positioning
pub fn draw_text(
  text: &str,
  font_style: &FontStyle,
  context: &FontContext,
  canvas: &mut Blend<RgbaImage>,
  layout: Layout,
) {
  let alpha = font_style.color.alpha();

  if alpha == 0.0 {
    return;
  }

  let content_box = layout.content_box_size();

  let start_x = layout.content_box_x();
  let start_y =
    layout.content_box_y() + font_style.font_size * ((font_style.line_height - 1.0) / 2.0);

  let metrics = Metrics::relative(font_style.font_size, font_style.line_height);
  let mut buffer = Buffer::new_empty(metrics);

  let mut attrs = Attrs::new().weight(font_style.font_weight.into());
  if let Some(font_family) = font_style.font_family.as_ref() {
    attrs = attrs.family(Family::Name(font_family));
  }

  let mut font_system = context.font_system.lock().unwrap();

  buffer.set_size(
    &mut font_system,
    Some(content_box.width),
    Some(content_box.height),
  );
  buffer.set_rich_text(
    &mut font_system,
    [(text, attrs.clone())],
    &attrs,
    Shaping::Advanced,
    Some(font_style.text_align.into()),
  );

  let mut font_cache = context.font_cache.lock().unwrap();

  buffer.draw(
    &mut font_system,
    &mut font_cache,
    font_style.color.into(),
    |x, y, w, h, color| {
      let color = Rgba([
        color.r(),
        color.g(),
        color.b(),
        (color.a() as f32 * alpha) as u8,
      ]);

      if color.0[3] == 0 {
        return;
      }

      draw_filled_rect_mut(
        canvas,
        Rect::at(start_x as i32 + x, start_y as i32 + y).of_size(w, h),
        color,
      );
    },
  );
}

/// Draws an image on the canvas with the specified style and layout.
///
/// The image will be resized if necessary to fit the content box, and border radius
/// will be applied if specified in the style.
///
/// # Arguments
/// * `image` - The image to draw
/// * `style` - The style to apply to the image
/// * `canvas` - The canvas to draw on
/// * `layout` - The layout information for positioning
pub fn draw_image(image: &RgbaImage, style: &Style, canvas: &mut Blend<RgbaImage>, layout: Layout) {
  let content_box = layout.content_box_size();
  let x = layout.content_box_x();
  let y = layout.content_box_y();

  let should_resize =
    content_box.width as u32 != image.width() || content_box.height as u32 != image.height();

  if !should_resize && style.inheritable_style.border_radius.is_none() {
    return overlay(&mut canvas.0, image, x as i64, y as i64);
  }

  let mut resized = resize(
    image,
    content_box.width as u32,
    content_box.height as u32,
    FilterType::Lanczos3,
  );

  if let Some(border_radius) = style.inheritable_style.border_radius {
    apply_border_radius_antialiased(&mut resized, border_radius);
  }

  overlay(&mut canvas.0, &resized, x as i64, y as i64);
}
