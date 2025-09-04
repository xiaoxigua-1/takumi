//! Canvas operations and image blending for the takumi rendering system.
//!
//! This module provides performance-optimized canvas operations including
//! fast image blending and pixel manipulation operations.

use std::{
  borrow::Cow,
  sync::{
    Arc,
    mpsc::{Receiver, Sender},
  },
};

use image::{Pixel, Rgba, RgbaImage};
use taffy::{Point, Size};
use zeno::{Mask, Placement, Transform};

use crate::{
  layout::{Viewport, style::Color},
  rendering::BorderRadius,
};

/// A canvas handle for sending drawing commands asynchronously.
///
/// This struct wraps a channel sender that can be cloned and used to send
/// drawing commands to a canvas rendering loop without blocking the main thread.
#[derive(Clone)]
pub struct Canvas(Sender<DrawCommand>);

impl Canvas {
  /// Creates a new canvas handle from a draw command sender.
  ///
  /// # Arguments
  /// * `sender` - The channel sender for sending drawing commands
  ///
  /// # Returns
  /// A new `Canvas` instance that can be used to send drawing commands
  pub fn new(sender: Sender<DrawCommand>) -> Self {
    Self(sender)
  }

  /// Overlays an image onto the canvas with optional border radius.
  ///
  /// # Arguments
  /// * `image` - The image to overlay on the canvas
  /// * `offset` - The position offset where to place the image
  /// * `radius` - Border radius to apply to the image corners
  pub fn overlay_image(
    &self,
    image: Arc<RgbaImage>,
    offset: Point<i32>,
    radius: BorderRadius,
    transform: Transform,
  ) {
    let _ = self.0.send(DrawCommand::OverlayImage {
      image,
      offset,
      radius,
      transform,
    });
  }

  /// Draws a mask with the specified color onto the canvas.
  pub fn draw_mask(
    &self,
    mask: Vec<u8>,
    placement: Placement,
    color: Color,
    image: Option<RgbaImage>,
  ) {
    let _ = self.0.send(DrawCommand::DrawMask {
      mask,
      placement,
      color,
      image,
    });
  }

  /// Fills a rectangular area with the specified color and optional border radius.
  ///
  /// # Arguments
  /// * `offset` - The position offset where to start filling
  /// * `size` - The size of the area to fill
  /// * `color` - The color to fill the area with
  /// * `radius` - Border radius to apply to the filled area
  pub fn fill_color(
    &self,
    offset: Point<i32>,
    size: Size<u32>,
    color: Color,
    radius: BorderRadius,
    transform: Transform,
  ) {
    let _ = self.0.send(DrawCommand::FillColor {
      offset,
      size,
      color,
      radius,
      transform,
    });
  }
}

/// A canvas that receives draw tasks from the main rendering thread and draws them to the canvas.
pub fn create_blocking_canvas_loop(
  viewport: Viewport,
  receiver: Receiver<DrawCommand>,
) -> RgbaImage {
  let mut canvas = RgbaImage::new(viewport.width, viewport.height);

  while let Ok(task) = receiver.recv() {
    task.draw(&mut canvas);
  }

  canvas
}

/// Drawing commands that can be sent to a canvas for rendering.
///
/// These commands represent different types of drawing operations that can be
/// performed on a canvas, such as overlaying images, drawing masks, or filling areas.
pub enum DrawCommand {
  /// Overlay an image onto the canvas with optional border radius.
  OverlayImage {
    /// The image to overlay on the canvas
    image: Arc<RgbaImage>,
    /// The position offset where to place the image
    offset: Point<i32>,
    /// Border radius to apply to the image corners
    radius: BorderRadius,
    /// Transform to apply when drawing
    transform: Transform,
  },
  /// Draw a mask with the specified color onto the canvas.
  DrawMask {
    /// The mask data as a vector of alpha values (0-255)
    mask: Vec<u8>,
    /// The placement of the mask
    placement: Placement,
    /// The color to apply to the mask
    color: Color,
    /// The image to sample colors from
    image: Option<RgbaImage>,
  },
  /// Fill a rectangular area with the specified color and optional border radius.
  FillColor {
    /// The position offset where to start filling
    offset: Point<i32>,
    /// The size of the area to fill
    size: Size<u32>,
    /// The color to fill the area with
    color: Color,
    /// Border radius to apply to the filled area
    radius: BorderRadius,
    /// Transform to apply when drawing
    transform: Transform,
  },
}

impl DrawCommand {
  /// Executes the drawing command on the provided canvas.
  ///
  /// # Arguments
  /// * `canvas` - The canvas to draw on
  pub fn draw(&self, canvas: &mut RgbaImage) {
    match *self {
      DrawCommand::OverlayImage {
        ref image,
        offset,
        radius,
        transform,
      } => overlay_image(canvas, image, offset, radius, transform),
      DrawCommand::FillColor {
        offset,
        size,
        color,
        radius,
        transform,
      } => draw_filled_rect_color(canvas, size, offset, color, radius, transform),
      DrawCommand::DrawMask {
        ref mask,
        placement,
        color,
        ref image,
      } => draw_mask(canvas, mask, placement, color, image.as_ref()),
    }
  }
}

fn is_transform_identity(transform: Transform) -> bool {
  transform.xx == 1.0
    && transform.xy == 0.0
    && transform.yx == 0.0
    && transform.yy == 1.0
    && transform.x == 0.0
    && transform.y == 0.0
}

/// Draws a single pixel on the canvas with alpha blending.
///
/// If the color is fully transparent (alpha = 0), no operation is performed.
/// Otherwise, the pixel is blended with the existing canvas pixel using alpha blending.
///
/// # Arguments
/// * `canvas` - The canvas to draw on
/// * `x` - The x coordinate of the pixel
/// * `y` - The y coordinate of the pixel
/// * `color` - The color to draw (RGBA format)
pub fn draw_pixel(canvas: &mut RgbaImage, x: u32, y: u32, color: Rgba<u8>) {
  if color.0[3] == 0 {
    return;
  }

  // image-rs blend will skip the operation if the source color is fully transparent
  if let Some(pixel) = canvas.get_pixel_mut_checked(x, y) {
    pixel.blend(&color);
  }
}

pub(crate) fn apply_mask_alpha_to_pixel(pixel: Rgba<u8>, alpha: u8) -> Rgba<u8> {
  if alpha == u8::MAX {
    pixel
  } else {
    Rgba([
      pixel.0[0],
      pixel.0[1],
      pixel.0[2],
      (pixel.0[3] as f32 * (alpha as f32 / 255.0)) as u8,
    ])
  }
}

fn is_point_in_bounds(x: f32, y: f32, width: f32, height: f32) -> bool {
  x >= 0.0 && y >= 0.0 && x < width && y < height
}

/// Draws a filled rectangle with a solid color.
pub(crate) fn draw_filled_rect_color<C: Into<Rgba<u8>>>(
  image: &mut RgbaImage,
  size: Size<u32>,
  offset: Point<i32>,
  color: C,
  radius: BorderRadius,
  transform: Transform,
) {
  let color: Rgba<u8> = color.into();
  let can_direct_draw = is_transform_identity(transform) && radius.is_zero();

  // Fast path: if drawing on the entire canvas, we can just replace the entire canvas with the color
  if can_direct_draw
    && color.0[3] == 255
    && offset.x == 0
    && offset.y == 0
    && size.width == image.width()
    && size.height == image.height()
  {
    let image_mut = image.as_mut();
    let image_len = image_mut.len();

    for i in (0..image_len).step_by(4) {
      image_mut[i..i + 4].copy_from_slice(&color.0);
    }

    return;
  }

  // Fast path: if drawing on the entire canvas, we can just replace the entire canvas with the color
  if can_direct_draw {
    for y in 0..size.height {
      for x in 0..size.width {
        let dest_x = x as i32 + offset.x;
        let dest_y = y as i32 + offset.y;

        if dest_x < 0 || dest_y < 0 {
          continue;
        }

        draw_pixel(image, dest_x as u32, dest_y as u32, color);
      }
    }

    return;
  }

  let mut paths = Vec::new();

  radius.write_mask_commands(&mut paths);

  let mut mask = Mask::new(&paths);

  mask.transform(Some(transform));

  let (mask, mut placement) = mask.render();

  placement.left += offset.x;
  placement.top += offset.y;

  draw_mask(image, &mask, placement, color, None);
}

fn draw_mask<C: Into<Rgba<u8>>>(
  canvas: &mut RgbaImage,
  mask: &[u8],
  placement: Placement,
  color: C,
  image: Option<&RgbaImage>,
) {
  let color: Rgba<u8> = color.into();
  let mut i = 0;

  for y in 0..placement.height {
    for x in 0..placement.width {
      let alpha = mask[i];
      i += 1;

      if alpha == 0 {
        continue;
      }

      let dest_x = x as i32 + placement.left;
      let dest_y = y as i32 + placement.top;

      if dest_x < 0 || dest_y < 0 {
        continue;
      }

      let pixel = image
        .map(|image| {
          let pixel = *image.get_pixel(x, y);
          apply_mask_alpha_to_pixel(pixel, alpha)
        })
        .unwrap_or_else(|| apply_mask_alpha_to_pixel(color, alpha));

      draw_pixel(canvas, dest_x as u32, dest_y as u32, pixel);
    }
  }
}

pub(crate) fn overlay_image(
  canvas: &mut RgbaImage,
  image: &RgbaImage,
  offset: Point<i32>,
  radius: BorderRadius,
  transform: Transform,
) {
  if is_transform_identity(transform) && radius.is_zero() {
    for y in 0..image.height() {
      for x in 0..image.width() {
        let dest_x = offset.x + x as i32;
        let dest_y = offset.y + y as i32;

        if dest_x < 0 || dest_y < 0 {
          continue;
        }

        draw_pixel(canvas, dest_x as u32, dest_y as u32, *image.get_pixel(x, y));
      }
    }

    return;
  }

  let mut paths = Vec::new();

  radius.write_mask_commands(&mut paths);

  let mut mask = Mask::new(&paths);

  mask.transform(Some(transform));

  let (mask, mut placement) = mask.render();

  // Fast path: if only the radius needs to be applied, we can draw the mask directly
  if is_transform_identity(transform) {
    placement.left += offset.x;
    placement.top += offset.y;

    return draw_mask(canvas, &mask, placement, Color::transparent(), Some(image));
  }

  let mut image_cow = Cow::Borrowed(image);

  if !radius.is_zero() {
    let mut bottom_image = RgbaImage::new(image.width(), image.height());

    overlay_image(
      &mut bottom_image,
      image,
      Point::default(),
      radius,
      Transform::default(),
    );

    image_cow = Cow::Owned(bottom_image);
  }

  let transformed_image = transform_image(image_cow.as_ref(), transform, &mask, placement);

  placement.left += offset.x;
  placement.top += offset.y;

  draw_mask(
    canvas,
    &mask,
    placement,
    Color::transparent(),
    Some(&transformed_image),
  );
}

fn transform_image(
  image: &RgbaImage,
  transform: Transform,
  mask: &[u8],
  placement: Placement,
) -> RgbaImage {
  let inverse = match transform.invert() {
    Some(inv) => inv,
    None => return RgbaImage::new(0, 0),
  };

  let mut rotated_image = RgbaImage::new(placement.width, placement.height);

  let mut i = 0;

  for y in 0..placement.height {
    for x in 0..placement.width {
      let alpha = mask[i];
      i += 1;

      if alpha == 0 {
        continue;
      }

      let point = inverse.transform_vector(zeno::Point::new(x as f32, y as f32));

      let pixel = sample_bilinear(image, point.x, point.y);
      rotated_image.put_pixel(x, y, pixel);
    }
  }

  rotated_image
}

fn sample_bilinear(image: &RgbaImage, x: f32, y: f32) -> Rgba<u8> {
  let (width, height) = image.dimensions();

  // Check bounds
  if !is_point_in_bounds(x, y, width as f32, height as f32) {
    return Rgba([0, 0, 0, 100]);
  }

  // Get the four surrounding pixels
  let x0 = x.floor() as u32;
  let y0 = y.floor() as u32;
  let x1 = (x0 + 1).min(width - 1);
  let y1 = (y0 + 1).min(height - 1);

  // Calculate interpolation weights
  let fx = x - x0 as f32;
  let fy = y - y0 as f32;

  // Get the four corner pixels
  let p00 = image.get_pixel(x0, y0);
  let p10 = image.get_pixel(x1, y0);
  let p01 = image.get_pixel(x0, y1);
  let p11 = image.get_pixel(x1, y1);

  // Perform bilinear interpolation for each channel
  let mut result = [0u8; 4];
  for (i, value) in result.iter_mut().enumerate() {
    let top = p00.0[i] as f32 * (1.0 - fx) + p10.0[i] as f32 * fx;
    let bottom = p01.0[i] as f32 * (1.0 - fx) + p11.0[i] as f32 * fx;
    *value = (top * (1.0 - fy) + bottom * fy).round() as u8;
  }

  Rgba(result)
}
