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
  pub fn overlay_image(&self, image: Arc<RgbaImage>, offset: Point<i32>, radius: BorderRadius) {
    let _ = self.0.send(DrawCommand::OverlayImage {
      image,
      offset,
      radius,
    });
  }

  /// Draws a mask with the specified color onto the canvas.
  ///
  /// # Arguments
  /// * `mask` - The mask data as a vector of alpha values (0-255)
  /// * `offset` - The position offset where to place the mask
  /// * `size` - The size of the mask area
  /// * `color` - The color to apply to the mask
  pub fn draw_mask(&self, mask: Vec<u8>, offset: Point<i32>, size: Size<u32>, color: Color) {
    let _ = self.0.send(DrawCommand::DrawMask {
      mask,
      offset,
      size,
      color,
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
  ) {
    let _ = self.0.send(DrawCommand::FillColor {
      offset,
      size,
      color,
      radius,
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
      } => overlay_image(canvas, image, offset, radius),
      DrawCommand::FillColor {
        offset,
        size,
        color,
        radius,
      } => draw_filled_rect_color(canvas, size, offset, color, radius),
      DrawCommand::DrawMask {
        ref mask,
        offset,
        size,
        color,
      } => draw_mask(canvas, mask, offset, size, color),
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
  canvas.get_pixel_mut(x, y).blend(&color);
}

fn draw_mask(
  canvas: &mut RgbaImage,
  mask: &[u8],
  offset: Point<i32>,
  size: Size<u32>,
  color: Color,
) {
  let mut i = 0;

  for y in 0..size.height {
    for x in 0..size.width {
      if mask[i] == 0 {
        i += 1;
        continue;
      }

      let x = x as i32 + offset.x;
      let y = y as i32 + offset.y;

      if x < 0 || y < 0 || x >= canvas.width() as i32 || y >= canvas.height() as i32 {
        i += 1;
        continue;
      }

      let pixel = Rgba([color.0[0], color.0[1], color.0[2], mask[i]]);
      draw_pixel(canvas, x as u32, y as u32, pixel);
    }
  }
}

fn overlay_image(
  canvas: &mut RgbaImage,
  image: &RgbaImage,
  offset: Point<i32>,
  radius: BorderRadius,
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
    for y in 0..drawable_height {
      for x in 0..drawable_width {
        let pixel = *image.get_pixel(x + overlay_x, y + overlay_y);
        draw_pixel(canvas, x + draw_x, y + draw_y, pixel);
      }
    }

    return;
  }

  let mut paths = Vec::new();

  radius.write_mask_commands(&mut paths);

  let (mask, placement) = Mask::new(&paths).render();

  let mut i = 0;

  for y in 0..placement.height {
    for x in 0..placement.width {
      if mask[i] == 0 {
        i += 1;
        continue;
      }

      let x = x as i32 + placement.left;
      let y = y as i32 + placement.top;

      if x < 0 || y < 0 || x >= canvas.width() as i32 || y >= canvas.height() as i32 {
        i += 1;
        continue;
      }

      let pixel = *image.get_pixel(x as u32 + overlay_x, y as u32 + overlay_y);
      draw_pixel(canvas, x as u32 + draw_x, y as u32 + draw_y, pixel);
    }
  }
}
