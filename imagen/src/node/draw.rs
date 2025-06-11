use cosmic_text::{Attrs, Buffer, Family, Metrics, Shaping};
use image::{
  GenericImageView, ImageError, Rgba, RgbaImage,
  imageops::{FilterType, crop_imm, resize},
};
use imageproc::drawing::{Blend, Canvas};
use imageproc::{
  drawing::{draw_filled_rect_mut, draw_hollow_rect_mut},
  rect::Rect,
};
use taffy::Layout;

use crate::{
  border_radius::apply_border_radius_antialiased,
  color::Color,
  context::FontContext,
  node::style::{FontStyle, ObjectFit, Style},
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
/// The image will be resized and positioned according to the object_fit style property.
/// Border radius will be applied if specified in the style.
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

  let container_width = content_box.width as u32;
  let container_height = content_box.height as u32;
  let image_width = image.width();
  let image_height = image.height();

  let (mut processed_image, offset_x, offset_y) = match style.object_fit {
    ObjectFit::Fill => {
      // Fill: stretch the image to fill the container exactly
      let resized = resize(
        image,
        container_width,
        container_height,
        FilterType::Lanczos3,
      );
      (resized, 0, 0)
    }
    ObjectFit::Contain => {
      // Contain: scale the image to fit within the container while preserving aspect ratio
      let scale_x = container_width as f32 / image_width as f32;
      let scale_y = container_height as f32 / image_height as f32;
      let scale = scale_x.min(scale_y);

      let new_width = (image_width as f32 * scale) as u32;
      let new_height = (image_height as f32 * scale) as u32;

      let resized = resize(image, new_width, new_height, FilterType::Lanczos3);
      let offset_x = (container_width.saturating_sub(new_width)) / 2;
      let offset_y = (container_height.saturating_sub(new_height)) / 2;

      (resized, offset_x, offset_y)
    }
    ObjectFit::Cover => {
      // Cover: scale the image to cover the entire container while preserving aspect ratio
      let scale_x = container_width as f32 / image_width as f32;
      let scale_y = container_height as f32 / image_height as f32;
      let scale = scale_x.max(scale_y);

      let new_width = (image_width as f32 * scale) as u32;
      let new_height = (image_height as f32 * scale) as u32;

      let resized = resize(image, new_width, new_height, FilterType::Lanczos3);

      // Crop to fit container
      let crop_x = (new_width.saturating_sub(container_width)) / 2;
      let crop_y = (new_height.saturating_sub(container_height)) / 2;

      let cropped =
        crop_imm(&resized, crop_x, crop_y, container_width, container_height).to_image();
      (cropped, 0, 0)
    }
    ObjectFit::ScaleDown => {
      // ScaleDown: same as contain, but never scale up
      let scale_x = container_width as f32 / image_width as f32;
      let scale_y = container_height as f32 / image_height as f32;
      let scale = scale_x.min(scale_y).min(1.0); // Never scale up

      let new_width = (image_width as f32 * scale) as u32;
      let new_height = (image_height as f32 * scale) as u32;

      let resized = if scale < 1.0 {
        resize(image, new_width, new_height, FilterType::Lanczos3)
      } else {
        image.clone()
      };

      let offset_x = (container_width.saturating_sub(new_width)) / 2;
      let offset_y = (container_height.saturating_sub(new_height)) / 2;

      (resized, offset_x, offset_y)
    }
    ObjectFit::None => {
      // None: display the image at its natural size, centered, but crop if too large
      if image_width <= container_width && image_height <= container_height {
        // Image fits within container, center it
        let offset_x = (container_width - image_width) / 2;
        let offset_y = (container_height - image_height) / 2;
        (image.clone(), offset_x, offset_y)
      } else {
        // Image is larger than container, crop from center
        let crop_x = if image_width > container_width {
          (image_width - container_width) / 2
        } else {
          0
        };
        let crop_y = if image_height > container_height {
          (image_height - container_height) / 2
        } else {
          0
        };

        let crop_width = container_width.min(image_width);
        let crop_height = container_height.min(image_height);

        let cropped = crop_imm(image, crop_x, crop_y, crop_width, crop_height).to_image();

        let offset_x = if crop_width < container_width {
          (container_width - crop_width) / 2
        } else {
          0
        };
        let offset_y = if crop_height < container_height {
          (container_height - crop_height) / 2
        } else {
          0
        };

        (cropped, offset_x, offset_y)
      }
    }
  };

  // Apply border radius if specified
  if let Some(border_radius) = style.inheritable_style.border_radius {
    apply_border_radius_antialiased(&mut processed_image, border_radius);
  }

  draw_image_overlay_fast(
    canvas,
    &processed_image,
    offset_x + x as u32,
    offset_y + y as u32,
  );
}

/// Draws an image onto the canvas without bounds checking.
pub(crate) fn draw_image_overlay_fast(
  canvas: &mut Blend<RgbaImage>,
  image: &RgbaImage,
  left: u32,
  top: u32,
) {
  for y in 0..image.height() {
    for x in 0..image.width() {
      let pixel = unsafe { image.unsafe_get_pixel(x, y) };

      if pixel.0[3] == 0 {
        continue;
      }

      if pixel.0[3] == 255 {
        canvas.0.draw_pixel(x + left, y + top, pixel);
        continue;
      }

      canvas.draw_pixel(x + left, y + top, pixel);
    }
  }
}

/// Draws a solid color background on the canvas.
///
/// # Arguments
/// * `color` - The color to fill with
/// * `canvas` - The canvas to draw on
/// * `layout` - The layout information for positioning and size
pub fn draw_background_color(color: Color, canvas: &mut Blend<RgbaImage>, layout: Layout) {
  let rect = Rect::at(layout.location.x as i32, layout.location.y as i32)
    .of_size(layout.size.width as u32, layout.size.height as u32);

  draw_filled_rect_mut(canvas, rect, color.into());
}

/// Draws debug borders around the node's layout areas.
///
/// This function draws colored rectangles to visualize the content box
/// (red) and the full layout box (green) for debugging purposes.
///
/// # Arguments
/// * `canvas` - The canvas to draw on
/// * `layout` - The layout information for the node
pub fn draw_debug_border(canvas: &mut Blend<RgbaImage>, layout: Layout) {
  let x = layout.content_box_x();
  let y = layout.content_box_y();
  let size = layout.content_box_size();

  draw_hollow_rect_mut(
    canvas,
    Rect::at(x as i32, y as i32).of_size(size.width as u32, size.height as u32),
    Rgba([255, 0, 0, 100]),
  );

  draw_hollow_rect_mut(
    canvas,
    Rect::at(layout.location.x as i32, layout.location.y as i32)
      .of_size(layout.size.width as u32, layout.size.height as u32),
    Rgba([0, 255, 0, 100]),
  );
}
