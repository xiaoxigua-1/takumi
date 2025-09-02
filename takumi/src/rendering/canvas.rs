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
use zeno::Mask;

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
    transform_origin: Point<i32>,
    rotation: f32,
  ) {
    let _ = self.0.send(DrawCommand::OverlayImage {
      image,
      offset,
      radius,
      transform_origin,
      rotation,
    });
  }

  /// Draws a mask with the specified color onto the canvas.
  ///
  /// # Arguments
  /// * `mask` - The mask data as a vector of alpha values (0-255)
  /// * `offset` - The position offset where to place the mask
  /// * `size` - The size of the mask area
  /// * `color` - The color to apply to the mask
  pub fn draw_mask(
    &self,
    mask: Vec<u8>,
    offset: Point<i32>,
    size: Size<u32>,
    color: Color,
    transform_origin: Point<i32>,
    rotation: f32,
  ) {
    let _ = self.0.send(DrawCommand::DrawMask {
      mask,
      offset,
      size,
      color,
      transform_origin,
      rotation,
    });
  }

  /// Draws a mask using an image as the source of colors.
  pub fn draw_mask_with_image(
    &self,
    mask: Vec<u8>,
    offset: Point<i32>,
    size: Size<u32>,
    image: Arc<RgbaImage>,
    src_offset: Point<i32>,
  ) {
    let _ = self.0.send(DrawCommand::DrawMaskWithImage {
      mask,
      offset,
      size,
      image,
      src_offset,
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
    rotation: f32,
  ) {
    let _ = self.0.send(DrawCommand::FillColor {
      offset,
      size,
      color,
      radius,
      rotation,
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
    /// Rotation origin in canvas coordinates
    transform_origin: Point<i32>,
    /// Rotation in degrees to apply when drawing
    rotation: f32,
  },
  /// Draw a mask with the specified color onto the canvas.
  DrawMask {
    /// The mask data as a vector of alpha values (0-255)
    mask: Vec<u8>,
    /// The position offset where to place the mask
    offset: Point<i32>,
    /// The size of the mask area
    size: Size<u32>,
    /// The color to apply to the mask
    color: Color,
    /// Rotation origin in canvas coordinates
    transform_origin: Point<i32>,
    /// Rotation in degrees to apply when drawing
    rotation: f32,
  },
  /// Draw a mask using an image as the color source.
  DrawMaskWithImage {
    /// The mask data as a vector of alpha values (0-255)
    mask: Vec<u8>,
    /// The position offset where to place the mask
    offset: Point<i32>,
    /// The size of the mask area
    size: Size<u32>,
    /// The source image to sample colors from
    image: Arc<RgbaImage>,
    /// The source offset in `image` corresponding to `offset` on the canvas
    src_offset: Point<i32>,
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
    /// Rotation in degrees to apply when drawing
    rotation: f32,
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
        transform_origin,
        rotation,
      } => overlay_image(canvas, image, offset, radius, transform_origin, rotation),
      DrawCommand::FillColor {
        offset,
        size,
        color,
        radius,
        rotation,
      } => draw_filled_rect_color(canvas, size, offset, color, radius, rotation),
      DrawCommand::DrawMask {
        ref mask,
        offset,
        size,
        color,
        transform_origin,
        rotation,
      } => draw_mask(
        canvas,
        mask,
        offset,
        size,
        color,
        transform_origin,
        rotation,
      ),
      DrawCommand::DrawMaskWithImage {
        ref mask,
        offset,
        size,
        ref image,
        src_offset,
      } => draw_mask_with_image(canvas, mask, offset, size, image, src_offset),
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

fn get_canvas_size(canvas: &RgbaImage) -> Size<u32> {
  Size {
    width: canvas.width(),
    height: canvas.height(),
  }
}

fn apply_mask_alpha_to_pixel(pixel: Rgba<u8>, alpha: u8) -> Rgba<u8> {
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

fn calculate_mask_index(mask_x: f32, mask_y: f32, width: u32) -> usize {
  (mask_y.floor() as u32 * width + mask_x.floor() as u32) as usize
}

fn is_point_in_bounds(x: f32, y: f32, width: f32, height: f32) -> bool {
  x >= 0.0 && y >= 0.0 && x < width && y < height
}

fn iterate_rotated_pixels<F>(
  canvas_size: Size<u32>,
  offset: Point<i32>,
  size: Size<u32>,
  transform_origin: Point<i32>,
  rotation: f32,
  mut pixel_fn: F,
) where
  F: FnMut(i32, i32, f32, f32),
{
  let (min_x, min_y, max_x, max_y) =
    rotated_bounding_box(offset, size, transform_origin, canvas_size, rotation);

  for dest_y in min_y..=max_y {
    for dest_x in min_x..=max_x {
      let (src_x, src_y) = inverse_rotate(
        Point {
          x: dest_x,
          y: dest_y,
        },
        transform_origin,
        rotation,
      );

      pixel_fn(dest_x, dest_y, src_x, src_y);
    }
  }
}

fn draw_mask(
  canvas: &mut RgbaImage,
  mask: &[u8],
  offset: Point<i32>,
  size: Size<u32>,
  color: Color,
  transform_origin: Point<i32>,
  rotation: f32,
) {
  if rotation == 0.0 {
    let mut i = 0;
    for y in 0..size.height {
      for x in 0..size.width {
        let alpha = mask[i];
        i += 1;

        if alpha == 0 {
          continue;
        }

        let dest_x = x as i32 + offset.x;
        let dest_y = y as i32 + offset.y;

        if dest_x < 0 || dest_y < 0 {
          continue;
        }

        let pixel = Rgba([color.0[0], color.0[1], color.0[2], alpha]);
        draw_pixel(canvas, dest_x as u32, dest_y as u32, pixel);
      }
    }
    return;
  }

  // Use inverse rotation to avoid pixel gaps
  let canvas_size = get_canvas_size(canvas);
  iterate_rotated_pixels(
    canvas_size,
    offset,
    size,
    transform_origin,
    rotation,
    |dest_x, dest_y, src_x, src_y| {
      let mask_x = src_x - offset.x as f32;
      let mask_y = src_y - offset.y as f32;

      if !is_point_in_bounds(mask_x, mask_y, size.width as f32, size.height as f32) {
        return;
      }

      let mask_idx = calculate_mask_index(mask_x, mask_y, size.width);
      if mask_idx >= mask.len() {
        return;
      }

      let alpha = mask[mask_idx];
      if alpha == 0 {
        return;
      }

      let pixel = Rgba([color.0[0], color.0[1], color.0[2], alpha]);
      draw_pixel(canvas, dest_x as u32, dest_y as u32, pixel);
    },
  );
}

pub(crate) fn overlay_image(
  canvas: &mut RgbaImage,
  image: &RgbaImage,
  offset: Point<i32>,
  radius: BorderRadius,
  transform_origin: Point<i32>,
  rotation: f32,
) {
  let drawable_width = if offset.x < 0 {
    image
      .width()
      .saturating_sub(offset.x.saturating_neg() as u32)
  } else {
    image
      .width()
      .min(canvas.width().saturating_sub(offset.x as u32))
  };

  let drawable_height = if offset.y < 0 {
    image
      .height()
      .saturating_sub(offset.y.saturating_neg() as u32)
  } else {
    image
      .height()
      .min(canvas.height().saturating_sub(offset.y as u32))
  };

  if drawable_width == 0 || drawable_height == 0 {
    return;
  }

  let overlay_x = if offset.x < 0 {
    offset.x.saturating_neg() as u32
  } else {
    0
  };
  let overlay_y = if offset.y < 0 {
    offset.y.saturating_neg() as u32
  } else {
    0
  };

  let draw_x = offset.x.max(0) as u32;
  let draw_y = offset.y.max(0) as u32;

  if radius.is_zero() {
    if rotation == 0.0 {
      for y in 0..drawable_height {
        for x in 0..drawable_width {
          let pixel = *image.get_pixel(x + overlay_x, y + overlay_y);
          draw_pixel(canvas, x + draw_x, y + draw_y, pixel);
        }
      }
      return;
    }

    // Use inverse rotation to avoid pixel gaps
    let image_size = Size {
      width: drawable_width,
      height: drawable_height,
    };

    let image_offset = Point {
      x: draw_x as i32,
      y: draw_y as i32,
    };

    let canvas_size = get_canvas_size(canvas);
    iterate_rotated_pixels(
      canvas_size,
      image_offset,
      image_size,
      transform_origin,
      rotation,
      |dest_x, dest_y, src_x, src_y| {
        let img_x = src_x - draw_x as f32;
        let img_y = src_y - draw_y as f32;

        if !is_point_in_bounds(img_x, img_y, drawable_width as f32, drawable_height as f32) {
          return;
        }

        let src_img_x = (img_x.floor() as u32) + overlay_x;
        let src_img_y = (img_y.floor() as u32) + overlay_y;

        if src_img_x >= image.width() || src_img_y >= image.height() {
          return;
        }

        let pixel = *image.get_pixel(src_img_x, src_img_y);
        draw_pixel(canvas, dest_x as u32, dest_y as u32, pixel);
      },
    );

    return;
  }

  let mut paths = Vec::new();

  radius.write_mask_commands(&mut paths);

  let (mask, placement) = Mask::new(&paths).render();

  if rotation == 0.0 {
    let mut i = 0;
    for y in 0..placement.height {
      for x in 0..placement.width {
        let alpha = mask[i];
        i += 1;

        if alpha == 0 {
          continue;
        }

        let x = x as i32 + placement.left;
        let y = y as i32 + placement.top;

        if x < 0 || y < 0 {
          continue;
        }

        let Some(pixel) = image.get_pixel_checked(x as u32 + overlay_x, y as u32 + overlay_y)
        else {
          continue;
        };

        let pixel = apply_mask_alpha_to_pixel(*pixel, alpha);

        draw_pixel(canvas, x as u32 + draw_x, y as u32 + draw_y, pixel);
      }
    }
    return;
  }

  // Use inverse rotation for border radius case
  let mask_size = Size {
    width: placement.width,
    height: placement.height,
  };

  let mask_offset = Point {
    x: placement.left + draw_x as i32,
    y: placement.top + draw_y as i32,
  };

  let canvas_size = get_canvas_size(canvas);
  iterate_rotated_pixels(
    canvas_size,
    mask_offset,
    mask_size,
    transform_origin,
    rotation,
    |dest_x, dest_y, src_x, src_y| {
      let mask_x = src_x - (placement.left + draw_x as i32) as f32;
      let mask_y = src_y - (placement.top + draw_y as i32) as f32;

      if !is_point_in_bounds(
        mask_x,
        mask_y,
        placement.width as f32,
        placement.height as f32,
      ) {
        return;
      }

      let mask_idx = calculate_mask_index(mask_x, mask_y, placement.width);
      if mask_idx >= mask.len() {
        return;
      }

      let alpha = mask[mask_idx];
      if alpha == 0 {
        return;
      }

      let img_src_x = src_x - draw_x as f32;
      let img_src_y = src_y - draw_y as f32;

      if !is_point_in_bounds(
        img_src_x,
        img_src_y,
        drawable_width as f32,
        drawable_height as f32,
      ) {
        return;
      }

      let src_img_x = (img_src_x.floor() as u32) + overlay_x;
      let src_img_y = (img_src_y.floor() as u32) + overlay_y;

      let Some(pixel) = image.get_pixel_checked(src_img_x, src_img_y) else {
        return;
      };

      let pixel = apply_mask_alpha_to_pixel(*pixel, alpha);
      draw_pixel(canvas, dest_x as u32, dest_y as u32, pixel);
    },
  );
}

pub(crate) fn rotate_position(
  point: Point<i32>,
  origin: Point<i32>,
  size: Size<u32>,
  rotation: f32,
) -> Point<i32> {
  if rotation == 0.0 {
    return point;
  }

  let theta = rotation.to_radians();
  let cos_t = theta.cos();
  let sin_t = theta.sin();

  let dx = point.x - origin.x;
  let dy = point.y - origin.y;

  let rx = origin.x as f32 + (dx as f32 * cos_t - dy as f32 * sin_t);
  let ry = origin.y as f32 + (dx as f32 * sin_t + dy as f32 * cos_t);

  // Clamp to canvas bounds to avoid overflow
  let rx_i = rx.round() as i32;
  let ry_i = ry.round() as i32;

  let max_x = size.width as i32 - 1;
  let max_y = size.height as i32 - 1;

  Point {
    x: rx_i.clamp(0, max_x),
    y: ry_i.clamp(0, max_y),
  }
}

/// Computes the axis-aligned bounding box of a rectangle after rotation, clamped to canvas bounds.
/// Returns (min_x, min_y, max_x, max_y) in integer canvas coordinates.
pub(crate) fn rotated_bounding_box(
  offset: Point<i32>,
  size: Size<u32>,
  origin: Point<i32>,
  canvas_size: Size<u32>,
  rotation: f32,
) -> (i32, i32, i32, i32) {
  if rotation == 0.0 {
    let min_x = offset.x.max(0);
    let min_y = offset.y.max(0);
    let max_x = (offset.x + size.width as i32 - 1).min(canvas_size.width as i32 - 1);
    let max_y = (offset.y + size.height as i32 - 1).min(canvas_size.height as i32 - 1);
    return (min_x, min_y, max_x, max_y);
  }

  let corners = [
    Point {
      x: offset.x,
      y: offset.y,
    },
    Point {
      x: offset.x + size.width as i32,
      y: offset.y,
    },
    Point {
      x: offset.x,
      y: offset.y + size.height as i32,
    },
    Point {
      x: offset.x + size.width as i32,
      y: offset.y + size.height as i32,
    },
  ];

  let mut min_x = i32::MAX;
  let mut min_y = i32::MAX;
  let mut max_x = i32::MIN;
  let mut max_y = i32::MIN;

  for &c in &corners {
    let rc = rotate_position(c, origin, canvas_size, rotation);
    min_x = min_x.min(rc.x);
    min_y = min_y.min(rc.y);
    max_x = max_x.max(rc.x);
    max_y = max_y.max(rc.y);
  }

  min_x = min_x.max(0);
  min_y = min_y.max(0);
  max_x = max_x.min(canvas_size.width as i32 - 1);
  max_y = max_y.min(canvas_size.height as i32 - 1);

  (min_x, min_y, max_x, max_y)
}

/// Applies the inverse rotation to a destination point, returning the corresponding source float coordinates.
pub(crate) fn inverse_rotate(point: Point<i32>, origin: Point<i32>, rotation: f32) -> (f32, f32) {
  if rotation == 0.0 {
    return (point.x as f32, point.y as f32);
  }

  let theta = (-rotation).to_radians();
  let cos_t = theta.cos();
  let sin_t = theta.sin();

  let vx = point.x - origin.x;
  let vy = point.y - origin.y;

  let sx = origin.x as f32 + (vx as f32 * cos_t - vy as f32 * sin_t);
  let sy = origin.y as f32 + (vx as f32 * sin_t + vy as f32 * cos_t);

  (sx, sy)
}

fn draw_mask_with_image(
  canvas: &mut RgbaImage,
  mask: &[u8],
  offset: Point<i32>,
  size: Size<u32>,
  image: &RgbaImage,
  src_offset: Point<i32>,
) {
  let mut i = 0;

  for y in 0..size.height {
    for x in 0..size.width {
      let alpha = mask[i];
      i += 1;

      if alpha == 0 {
        continue;
      }

      let dst_x = x as i32 + offset.x;
      let dst_y = y as i32 + offset.y;

      if dst_x < 0 || dst_y < 0 || dst_x >= canvas.width() as i32 || dst_y >= canvas.height() as i32
      {
        continue;
      }

      let src_x = x as i32 + src_offset.x;
      let src_y = y as i32 + src_offset.y;

      if src_x < 0 || src_y < 0 || src_x >= image.width() as i32 || src_y >= image.height() as i32 {
        continue;
      }

      let mut pixel = *image.get_pixel(src_x as u32, src_y as u32);
      if alpha != u8::MAX {
        pixel.0[3] = ((pixel.0[3] as f32) * (alpha as f32 / 255.0)) as u8;
      }

      draw_pixel(canvas, dst_x as u32, dst_y as u32, pixel);
    }
  }
}
