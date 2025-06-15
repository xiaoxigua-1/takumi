use cosmic_text::{Attrs, Buffer, Family, Metrics, Shaping};
use image::{
  GenericImageView, ImageError, Pixel, Rgba, RgbaImage,
  imageops::{FilterType, crop_imm, fast_blur, resize},
};
use imageproc::drawing::Canvas;
use imageproc::{
  drawing::{draw_filled_rect_mut, draw_hollow_rect_mut},
  rect::Rect,
};
use taffy::Layout;

use crate::{
  border_radius::{BorderRadius, apply_border_radius_antialiased},
  color::{ColorAt, ColorInput, Gradient},
  node::style::{BoxShadow, BoxShadowInput, ObjectFit, ResolvedFontStyle, Style},
  render::RenderContext,
};

/// A performance-optimized implementation of image blending operations.
///
/// This implementation provides faster blending by skipping pixel operations when the source color is fully transparent
/// and using direct pixel assignment when the source color is fully opaque.
///
/// Based on the implementation from [imageproc's Blend](https://docs.rs/imageproc/latest/imageproc/drawing/struct.Blend.html).
pub struct FastBlendImage(pub RgbaImage);

impl Canvas for FastBlendImage {
  type Pixel = Rgba<u8>;

  fn dimensions(&self) -> (u32, u32) {
    self.0.dimensions()
  }

  fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
    *self.0.get_pixel(x, y)
  }

  fn draw_pixel(&mut self, x: u32, y: u32, color: Self::Pixel) {
    if color.0[3] == 0 {
      return;
    }

    if color.0[3] == 255 {
      self.0.put_pixel(x, y, color);
      return;
    }

    let mut pix = *self.0.get_pixel(x, y);

    pix.blend(&color);

    self.0.put_pixel(x, y, pix);
  }
}

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
pub fn draw_text(
  text: &str,
  style: &ResolvedFontStyle,
  context: &RenderContext,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  if style.color.is_transparent() || style.font_size == 0.0 {
    return;
  }

  let content_box = layout.content_box_size();

  let start_x = layout.content_box_x();
  let start_y = layout.content_box_y() + style.font_size * ((style.line_height - 1.0) / 2.0);

  let mut buffer = construct_text_buffer(text, style, context);

  let mut font_system = context.global.font_context.font_system.lock().unwrap();

  buffer.set_size(
    &mut font_system,
    Some(content_box.width),
    Some(content_box.height),
  );

  let mut font_cache = context.global.font_context.font_cache.lock().unwrap();

  buffer.draw(
    &mut font_system,
    &mut font_cache,
    cosmic_text::Color(0),
    |x, y, w, h, color| {
      let color = color.as_rgba();

      let text_alpha = color[3] as f32 / 255.0;

      if text_alpha == 0.0 {
        return;
      }

      // FIXME: emojis with rich coloring with black might not be rendered correctly.
      let mut render_color: Rgba<u8> = if color[0] == 0 && color[1] == 0 && color[2] == 0 {
        style
          .color
          .at(content_box.width, content_box.height, x as u32, y as u32)
          .into()
      } else {
        Rgba(color)
      };

      render_color.0[3] = (render_color.0[3] as f32 * text_alpha) as u8;

      draw_filled_rect_mut(
        canvas,
        Rect::at(start_x as i32 + x, start_y as i32 + y).of_size(w, h),
        render_color,
      );
    },
  );
}

/// Draws an image on the canvas with the specified style and layout.
///
/// The image will be resized and positioned according to the object_fit style property.
/// Border radius will be applied if specified in the style.
pub fn draw_image(
  image: &RgbaImage,
  style: &Style,
  context: &RenderContext,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
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
    apply_border_radius_antialiased(
      &mut processed_image,
      BorderRadius::from_layout(context, &layout, border_radius.into()),
    );
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
  canvas: &mut FastBlendImage,
  image: &RgbaImage,
  left: u32,
  top: u32,
) {
  for y in 0..image.height() {
    for x in 0..image.width() {
      let pixel = unsafe { image.unsafe_get_pixel(x, y) };

      canvas.draw_pixel(x + left, y + top, pixel);
    }
  }
}

/// Draws a filled rectangle on the canvas from a color input.
pub fn draw_filled_rect_from_color_input(
  canvas: &mut FastBlendImage,
  rect: Rect,
  color: &ColorInput,
) {
  match color {
    ColorInput::Color(color) => {
      draw_filled_rect_mut(canvas, rect, (*color).into());
    }
    ColorInput::Gradient(gradient) => {
      let gradient_image = create_gradient_image(gradient, rect.width(), rect.height());

      draw_image_overlay_fast(
        canvas,
        &gradient_image,
        rect.left() as u32,
        rect.top() as u32,
      );
    }
  }
}

/// Creates an image from a gradient.
pub fn create_gradient_image(color: &Gradient, width: u32, height: u32) -> RgbaImage {
  RgbaImage::from_par_fn(width, height, |x, y| {
    color.at(width as f32, height as f32, x, y).into()
  })
}

/// Creates an image from a color input.
pub fn create_image_from_color_input(color: &ColorInput, width: u32, height: u32) -> RgbaImage {
  match color {
    ColorInput::Color(color) => {
      let color = *color;

      RgbaImage::from_pixel(width, height, color.into())
    }
    ColorInput::Gradient(gradient) => create_gradient_image(gradient, width, height),
  }
}

/// Draws a solid color background on the canvas.
pub fn draw_background_color(
  color: &ColorInput,
  radius: Option<BorderRadius>,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  let rect = Rect::at(layout.location.x as i32, layout.location.y as i32)
    .of_size(layout.size.width as u32, layout.size.height as u32);

  let Some(radius) = radius else {
    draw_filled_rect_from_color_input(canvas, rect, color);
    return;
  };

  let mut image = create_image_from_color_input(color, rect.width(), rect.height());

  apply_border_radius_antialiased(&mut image, radius);

  draw_image_overlay_fast(
    canvas,
    &image,
    layout.location.x as u32,
    layout.location.y as u32,
  );
}

/// Draws debug borders around the node's layout areas.
///
/// This function draws colored rectangles to visualize the content box
/// (red) and the full layout box (green) for debugging purposes.
pub fn draw_debug_border(canvas: &mut FastBlendImage, layout: Layout) {
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

/// Applies a fast blur to an image using image-rs's optimized implementation.
fn apply_fast_blur(image: &mut RgbaImage, radius: f32) {
  if radius <= 0.0 {
    return;
  }

  // Convert CSS blur radius to sigma for fast_blur
  // CSS blur radius is roughly 3x the standard deviation (sigma)
  let sigma = radius / 3.0;

  *image = fast_blur(image, sigma);
}

/// Draws box shadows for an element.
pub fn draw_box_shadow(
  box_shadow_input: &BoxShadowInput,
  style: &Style,
  context: &RenderContext,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  let shadows = match box_shadow_input {
    BoxShadowInput::Single(shadow) => vec![shadow],
    BoxShadowInput::Multiple(shadows) => shadows.iter().collect(),
  };

  // Draw shadows from back to front (reverse order)
  for shadow in shadows.iter().rev() {
    draw_single_box_shadow(shadow, style, context, canvas, layout);
  }
}

/// Draws a single box shadow.
fn draw_single_box_shadow(
  shadow: &BoxShadow,
  style: &Style,
  context: &RenderContext,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  if shadow.inset {
    draw_inset_shadow(shadow, style, context, canvas, layout);
  } else {
    draw_outset_shadow(shadow, style, context, canvas, layout);
  }
}

/// Draws an outset (external) box shadow.
fn draw_outset_shadow(
  shadow: &BoxShadow,
  style: &Style,
  context: &RenderContext,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  let shadow = shadow.clone().resolve(context);
  let blur_extent = shadow.blur_radius * 2.0; // Blur extends in all directions
  let spread = shadow.spread_radius;

  // Calculate the full shadow bounds including blur
  let shadow_x = layout.location.x + shadow.offset_x - spread - blur_extent;
  let shadow_y = layout.location.y + shadow.offset_y - spread - blur_extent;
  let shadow_width = layout.size.width + (spread + blur_extent) * 2.0;
  let shadow_height = layout.size.height + (spread + blur_extent) * 2.0;

  // Skip if completely outside canvas
  let canvas_dims = canvas.dimensions();
  if shadow_x + shadow_width < 0.0
    || shadow_y + shadow_height < 0.0
    || shadow_x >= canvas_dims.0 as f32
    || shadow_y >= canvas_dims.1 as f32
  {
    return;
  }

  // Create the base shadow shape (element + spread)
  let base_width = (layout.size.width + spread * 2.0) as u32;
  let base_height = (layout.size.height + spread * 2.0) as u32;
  let mut shadow_image = create_image_from_color_input(&shadow.color, base_width, base_height);

  // Apply border radius to shadow shape
  if let Some(border_radius) = style.inheritable_style.border_radius {
    let mut adjusted_radius = BorderRadius::from_layout(context, &layout, border_radius.into());

    adjusted_radius.top_left += spread;
    adjusted_radius.top_right += spread;
    adjusted_radius.bottom_right += spread;
    adjusted_radius.bottom_left += spread;

    apply_border_radius_antialiased(&mut shadow_image, adjusted_radius);
  }

  // Apply blur if needed
  if shadow.blur_radius > 0.0 {
    // Expand canvas for blur to prevent edge artifacts
    let blur_padding = (shadow.blur_radius * 2.0) as u32;
    let padded_width = base_width + blur_padding * 2;
    let padded_height = base_height + blur_padding * 2;

    let mut padded_image = RgbaImage::new(padded_width, padded_height);

    // Center the shadow in the padded image
    for y in 0..shadow_image.height() {
      for x in 0..shadow_image.width() {
        let pixel = *shadow_image.get_pixel(x, y);
        padded_image.put_pixel(x + blur_padding, y + blur_padding, pixel);
      }
    }

    apply_fast_blur(&mut padded_image, shadow.blur_radius);
    shadow_image = padded_image;
  }

  // Calculate final position accounting for blur expansion
  let final_x = if shadow.blur_radius > 0.0 {
    shadow_x
  } else {
    layout.location.x + shadow.offset_x - spread
  };
  let final_y = if shadow.blur_radius > 0.0 {
    shadow_y
  } else {
    layout.location.y + shadow.offset_y - spread
  };

  // Draw the shadow with clipping
  draw_image_with_clipping(canvas, &shadow_image, final_x as i32, final_y as i32);
}

/// Draws an inset (internal) box shadow.
fn draw_inset_shadow(
  shadow: &BoxShadow,
  style: &Style,
  context: &RenderContext,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  let shadow = shadow.clone().resolve(context);
  let content_box = layout.content_box_size();
  let element_x = layout.content_box_x();
  let element_y = layout.content_box_y();

  // Create element mask
  let mut element_mask = RgbaImage::from_pixel(
    content_box.width as u32,
    content_box.height as u32,
    Rgba([255, 255, 255, 255]),
  );

  // Apply border radius to mask
  if let Some(border_radius) = style.inheritable_style.border_radius {
    apply_border_radius_antialiased(
      &mut element_mask,
      BorderRadius::from_layout(context, &layout, border_radius.into()),
    );
  }

  // Create inverted shadow
  let blur_extent = shadow.blur_radius * 2.0;
  let shadow_width = (content_box.width + blur_extent * 2.0) as u32;
  let shadow_height = (content_box.height + blur_extent * 2.0) as u32;

  let mut shadow_image = RgbaImage::new(shadow_width, shadow_height);

  // Fill with shadow color, leaving hole for element
  for y in 0..shadow_height {
    for x in 0..shadow_width {
      let rel_x = x as f32 - blur_extent;
      let rel_y = y as f32 - blur_extent;

      // Check if we're inside the element bounds
      if rel_x >= -shadow.offset_x - shadow.spread_radius
        && rel_y >= -shadow.offset_y - shadow.spread_radius
        && rel_x < content_box.width - shadow.offset_x + shadow.spread_radius
        && rel_y < content_box.height - shadow.offset_y + shadow.spread_radius
      {
        let mask_x = (rel_x + shadow.offset_x + shadow.spread_radius) as u32;
        let mask_y = (rel_y + shadow.offset_y + shadow.spread_radius) as u32;

        if mask_x < element_mask.width() && mask_y < element_mask.height() {
          let mask_pixel = element_mask.get_pixel(mask_x, mask_y);
          if mask_pixel[3] == 0 {
            // Outside element shape, draw shadow
            let shadow_color: Rgba<u8> = match &shadow.color {
              ColorInput::Color(color) => (*color).into(),
              ColorInput::Gradient(_) => Rgba([0, 0, 0, 128]), // Fallback for gradients
            };
            shadow_image.put_pixel(x, y, shadow_color);
          }
        }
      }
    }
  }

  // Apply blur
  if shadow.blur_radius > 0.0 {
    apply_fast_blur(&mut shadow_image, shadow.blur_radius);
  }

  // Composite with element mask
  for y in 0..element_mask.height() {
    for x in 0..element_mask.width() {
      let mask_pixel = element_mask.get_pixel(x, y);
      if mask_pixel[3] > 0 {
        let shadow_x = x + blur_extent as u32;
        let shadow_y = y + blur_extent as u32;

        if shadow_x < shadow_image.width() && shadow_y < shadow_image.height() {
          let shadow_pixel = shadow_image.get_pixel(shadow_x, shadow_y);
          if shadow_pixel[3] > 0 {
            canvas.draw_pixel(element_x as u32 + x, element_y as u32 + y, *shadow_pixel);
          }
        }
      }
    }
  }
}

/// Draws an image with proper clipping to canvas bounds.
fn draw_image_with_clipping(canvas: &mut FastBlendImage, image: &RgbaImage, x: i32, y: i32) {
  let canvas_dims = canvas.dimensions();
  let (canvas_width, canvas_height) = (canvas_dims.0 as i32, canvas_dims.1 as i32);

  // Calculate clipping bounds
  let start_x = x.max(0);
  let start_y = y.max(0);
  let end_x = (x + image.width() as i32).min(canvas_width);
  let end_y = (y + image.height() as i32).min(canvas_height);

  // Draw only the visible portion
  for canvas_y in start_y..end_y {
    for canvas_x in start_x..end_x {
      let img_x = (canvas_x - x) as u32;
      let img_y = (canvas_y - y) as u32;

      if img_x < image.width() && img_y < image.height() {
        let pixel = *image.get_pixel(img_x, img_y);
        if pixel[3] > 0 {
          canvas.draw_pixel(canvas_x as u32, canvas_y as u32, pixel);
        }
      }
    }
  }
}

pub(crate) fn construct_text_buffer(
  text: &str,
  font_style: &ResolvedFontStyle,
  context: &RenderContext,
) -> Buffer {
  let metrics = Metrics::relative(font_style.font_size, font_style.line_height);
  let mut buffer = Buffer::new_empty(metrics);

  let mut attrs = Attrs::new().weight(font_style.font_weight);

  if let Some(font_family) = font_style.font_family.as_ref() {
    attrs = attrs.family(Family::Name(font_family));
  }

  if let Some(letter_spacing) = font_style.letter_spacing {
    attrs = attrs.letter_spacing(letter_spacing);
  }

  let mut font_system = context.global.font_context.font_system.lock().unwrap();

  buffer.set_rich_text(
    &mut font_system,
    [(text, attrs.clone())],
    &attrs,
    Shaping::Advanced,
    Some(font_style.text_align),
  );

  buffer
}
