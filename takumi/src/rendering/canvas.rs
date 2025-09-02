//! Canvas operations and image blending for the takumi rendering system.
//!
//! This module provides performance-optimized canvas operations including
//! fast image blending and pixel manipulation operations.

use std::sync::{
  Arc,
  mpsc::{Receiver, Sender},
};

use image::{Pixel, Rgba, RgbaImage};
use taffy::{Point, Size};
use zeno::{Mask, Placement, Transform};

use crate::{
  layout::{Viewport, style::Color},
  rendering::{BorderRadius, draw_filled_rect_color},
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
    transform: Option<Transform>,
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
    offset: Point<i32>,
    placement: Placement,
    color: Color,
    image: Option<Arc<RgbaImage>>,
  ) {
    let _ = self.0.send(DrawCommand::DrawMask {
      mask,
      offset,
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
    transform: Option<Transform>,
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
    transform: Option<Transform>,
  },
  /// Draw a mask with the specified color onto the canvas.
  DrawMask {
    /// The mask data as a vector of alpha values (0-255)
    mask: Vec<u8>,
    /// The position offset where to place the mask
    offset: Point<i32>,
    /// The placement of the mask
    placement: Placement,
    /// The color to apply to the mask
    color: Color,
    /// The image to sample colors from
    image: Option<Arc<RgbaImage>>,
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
    transform: Option<Transform>,
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
        offset,
        placement,
        color,
        ref image,
      } => draw_mask(
        canvas,
        mask,
        offset,
        placement,
        color,
        image.as_ref().map(|image| image.as_ref()),
      ),
    }
  }
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

fn draw_mask(
  canvas: &mut RgbaImage,
  mask: &[u8],
  offset: Point<i32>,
  placement: Placement,
  color: Color,
  image: Option<&RgbaImage>,
) {
  let mut i = 0;

  for y in 0..placement.height {
    for x in 0..placement.width {
      let alpha = mask[i];
      i += 1;

      if alpha == 0 {
        continue;
      }

      let dest_x = x as i32 + offset.x + placement.left;
      let dest_y = y as i32 + offset.y + placement.top;

      if dest_x < 0 || dest_y < 0 {
        continue;
      }

      let pixel = image
        .map(|image| {
          let pixel = *image.get_pixel(x as u32, y as u32);
          apply_mask_alpha_to_pixel(pixel, alpha)
        })
        .unwrap_or_else(|| apply_mask_alpha_to_pixel(color.0.into(), alpha));

      draw_pixel(canvas, dest_x as u32, dest_y as u32, pixel);
    }
  }
}

pub(crate) fn overlay_image(
  canvas: &mut RgbaImage,
  image: &RgbaImage,
  offset: Point<i32>,
  radius: BorderRadius,
  transform: Option<Transform>,
) {
  if transform.is_none() && radius.is_zero() {
    for y in 0..image.height() {
      for x in 0..image.width() {
        draw_pixel(canvas, x as u32, y as u32, *image.get_pixel(x, y));
      }
    }

    return;
  }

  let mut paths = Vec::new();

  radius.write_mask_commands(&mut paths);

  let mut mask = Mask::new(&paths);

  mask.transform(transform);

  let (mask, mut placement) = mask.render();

  let rotate_radians = transform.map(transform_rotation_radians).unwrap_or(0.0);

  if rotate_radians != 0.0 {
    let (rotated_image, rotated_offset) = rotate_image(image, rotate_radians);

    placement.left += rotated_offset.x as i32;
    placement.top += rotated_offset.y as i32;

    return draw_mask(
      canvas,
      &mask,
      offset,
      placement,
      Color::transparent(),
      Some(&rotated_image),
    );
  }

  draw_mask(
    canvas,
    &mask,
    offset,
    placement,
    Color::transparent(),
    Some(image),
  );
}

fn transform_rotation_radians(transform: Transform) -> f32 {
  transform.yx.atan2(transform.xx)
}

fn rotate_image(image: &RgbaImage, theta: f32) -> (RgbaImage, Point<f32>) {
  let (width, height) = image.dimensions();
  let center_x = width as f32 / 2.0;
  let center_y = height as f32 / 2.0;

  let cos_theta = theta.cos();
  let sin_theta = theta.sin();

  // Calculate the bounds of the rotated image by transforming corners
  let corners = [
    (0.0, 0.0),
    (width as f32, 0.0),
    (0.0, height as f32),
    (width as f32, height as f32),
  ];

  let mut min_x = f32::INFINITY;
  let mut max_x = f32::NEG_INFINITY;
  let mut min_y = f32::INFINITY;
  let mut max_y = f32::NEG_INFINITY;

  // Transform each corner to find the bounding box
  for &(x, y) in corners.iter() {
    // Translate to center, rotate, then translate back
    let centered_x = x - center_x;
    let centered_y = y - center_y;

    let rotated_x = centered_x * cos_theta - centered_y * sin_theta + center_x;
    let rotated_y = centered_x * sin_theta + centered_y * cos_theta + center_y;

    min_x = min_x.min(rotated_x);
    max_x = max_x.max(rotated_x);
    min_y = min_y.min(rotated_y);
    max_y = max_y.max(rotated_y);
  }

  // Calculate new dimensions and offset
  let new_width = (max_x - min_x).ceil() as u32;
  let new_height = (max_y - min_y).ceil() as u32;
  let offset_x = -min_x;
  let offset_y = -min_y;

  // Create the output image
  let mut rotated_image = RgbaImage::new(new_width, new_height);

  // Fill the rotated image using inverse transformation
  for y in 0..new_height {
    for x in 0..new_width {
      // Convert output coordinates to original image space
      let out_x = x as f32 - offset_x;
      let out_y = y as f32 - offset_y;

      // Translate to center
      let centered_x = out_x - center_x;
      let centered_y = out_y - center_y;

      // Apply inverse rotation (rotate by -theta)
      let orig_x = centered_x * cos_theta + centered_y * sin_theta + center_x;
      let orig_y = -centered_x * sin_theta + centered_y * cos_theta + center_y;

      // Use bilinear interpolation to sample from the original image
      let pixel = sample_bilinear(image, orig_x, orig_y);
      rotated_image.put_pixel(x, y, pixel);
    }
  }

  (
    rotated_image,
    Point {
      x: offset_x,
      y: offset_y,
    },
  )
}

fn sample_bilinear(image: &RgbaImage, x: f32, y: f32) -> Rgba<u8> {
  let (width, height) = image.dimensions();

  // Check bounds
  if !is_point_in_bounds(x, y, width as f32, height as f32) {
    return Rgba([0, 0, 0, 0]);
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
  for i in 0..4 {
    let top = p00.0[i] as f32 * (1.0 - fx) + p10.0[i] as f32 * fx;
    let bottom = p01.0[i] as f32 * (1.0 - fx) + p11.0[i] as f32 * fx;
    result[i] = (top * (1.0 - fy) + bottom * fy).round() as u8;
  }

  Rgba(result)
}
