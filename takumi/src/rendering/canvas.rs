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

use crate::{
  layout::{Viewport, style::Color},
  rendering::{BorderRadius, draw_filled_rect_color},
};

pub struct Canvas(Sender<DrawCommand>);

impl Canvas {
  pub fn new(sender: Sender<DrawCommand>) -> Self {
    Self(sender)
  }

  pub fn overlay_image(&self, image: Arc<RgbaImage>, offset: Point<i32>, radius: BorderRadius) {
    self.0.send(DrawCommand::OverlayImage {
      image,
      offset,
      radius,
    });
  }

  pub fn draw_mask(&self, mask: Vec<u8>, offset: Point<i32>, width: u32, height: u32) {
    self.0.send(DrawCommand::DrawMask {
      mask,
      offset,
      width,
      height,
    });
  }

  pub fn fill_color(
    &self,
    offset: Point<f32>,
    size: Size<f32>,
    color: Color,
    radius: BorderRadius,
  ) {
    self.0.send(DrawCommand::FillColor {
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

enum DrawCommand {
  OverlayImage {
    image: Arc<RgbaImage>,
    offset: Point<i32>,
    radius: BorderRadius,
  },
  DrawMask {
    mask: Vec<u8>,
    offset: Point<i32>,
    width: u32,
    height: u32,
  },
  FillColor {
    offset: Point<f32>,
    size: Size<f32>,
    color: Color,
    radius: BorderRadius,
  },
}

impl DrawCommand {
  pub fn draw(&self, canvas: &mut RgbaImage) {
    match *self {
      DrawCommand::OverlayImage {
        ref image,
        offset,
        radius,
      } => overlay_image(canvas, image, offset),
      DrawCommand::FillColor {
        offset,
        size,
        color,
        radius,
      } => draw_filled_rect_color(canvas, size, offset, color, radius),
      _ => todo!(),
    }
  }
}

pub fn draw_pixel(canvas: &mut RgbaImage, x: u32, y: u32, color: Rgba<u8>) {
  if color.0[3] == 0 {
    return;
  }

  // image-rs blend will skip the operation if the source color is fully transparent
  canvas.get_pixel_mut(x, y).blend(&color);
}

fn overlay_image(canvas: &mut RgbaImage, image: &RgbaImage, offset: Point<i32>) {
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

  for y in 0..drawable_height {
    for x in 0..drawable_width {
      let pixel = *image.get_pixel(x + overlay_x, y + overlay_y);
      draw_pixel(canvas, x + draw_x, y + draw_y, pixel);
    }
  }
}
