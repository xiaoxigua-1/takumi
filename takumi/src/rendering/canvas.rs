//! Canvas operations and image blending for the takumi rendering system.
//!
//! This module provides performance-optimized canvas operations including
//! fast image blending and pixel manipulation operations.

use std::{
  fmt::Display,
  sync::{
    Arc,
    mpsc::{Receiver, Sender},
  },
};

use image::{
  Pixel, Rgba, RgbaImage,
  imageops::{interpolate_bilinear, interpolate_nearest},
};
use taffy::{Point, Size};
use zeno::{Mask, Placement};

use crate::{
  layout::{
    Viewport,
    style::{Affine, Color, ImageScalingAlgorithm},
  },
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
    transform: Affine,
    algorithm: ImageScalingAlgorithm,
  ) {
    let _ = self.0.send(DrawCommand::OverlayImage {
      image,
      offset,
      radius,
      transform,
      algorithm,
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
    transform: Affine,
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

    println!("{task}");
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
    transform: Affine,
    /// The algorithm to use when transforming the image
    algorithm: ImageScalingAlgorithm,
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
    transform: Affine,
  },
}

impl Display for DrawCommand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      DrawCommand::OverlayImage {
        ref image,
        offset,
        radius,
        transform,
        algorithm,
      } => write!(
        f,
        "OverlayImage(width={}, height={}, offset={offset:?}, radius={radius:?}, transform={}, algorithm={algorithm:?})",
        image.width(),
        image.height(),
        transform.decompose()
      ),
      DrawCommand::FillColor {
        size,
        color,
        radius,
        transform,
        ..
      } => write!(
        f,
        "FillColor(size={size:?}, color={color}, radius={radius:?}, transform={})",
        transform.decompose()
      ),
      DrawCommand::DrawMask {
        placement, color, ..
      } => {
        write!(f, "DrawMask(placement={placement:?}, color={color})")
      }
    }
  }
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
        algorithm,
      } => overlay_image(canvas, image, offset, radius, transform, algorithm),
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

/// Draws a filled rectangle with a solid color.
pub(crate) fn draw_filled_rect_color<C: Into<Rgba<u8>>>(
  image: &mut RgbaImage,
  size: Size<u32>,
  offset: Point<i32>,
  color: C,
  radius: BorderRadius,
  transform: Affine,
) {
  let color: Rgba<u8> = color.into();
  let can_direct_draw = transform.is_identity() && radius.is_zero();

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
  transform.apply_on_paths(&mut paths);

  let mask = Mask::new(&paths);

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
  transform: Affine,
  algorithm: ImageScalingAlgorithm,
) {
  if transform.is_identity() && radius.is_zero() {
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

  if !radius.is_zero() {
    let mut paths = Vec::new();

    radius.write_mask_commands(&mut paths);

    let mask = Mask::new(&paths);

    let (mask, placement) = mask.render();

    let mut bottom = RgbaImage::new(image.width(), image.height());

    draw_mask(
      &mut bottom,
      &mask,
      placement,
      Rgba([0, 0, 0, 0]),
      Some(image),
    );

    return overlay_image(
      canvas,
      &bottom,
      offset,
      BorderRadius::zero(),
      transform,
      algorithm,
    );
  }

  draw_image_with_transform(canvas, image, transform, offset, algorithm);
}

fn draw_image_with_transform(
  canvas: &mut RgbaImage,
  image: &RgbaImage,
  transform: Affine,
  offset: Point<i32>,
  algorithm: ImageScalingAlgorithm,
) {
  let Some(inverse) = transform.invert() else {
    return;
  };

  let corners = [
    (0.0, 0.0),
    (image.width() as f32, 0.0),
    (image.width() as f32, image.height() as f32),
    (0.0, image.height() as f32),
  ];

  let corners_transformed = corners
    .into_iter()
    .map(|(x, y)| {
      let point = Point { x, y } * transform;
      (point.x, point.y)
    })
    .collect::<Vec<_>>();

  let mut min_x = f32::MAX;
  let mut min_y = f32::MAX;
  let mut max_x = f32::MIN;
  let mut max_y = f32::MIN;

  for (x, y) in corners_transformed {
    min_x = min_x.min(x);
    min_y = min_y.min(y);
    max_x = max_x.max(x);
    max_y = max_y.max(y);
  }

  let start_x = min_x.floor() as i32;
  let start_y = min_y.floor() as i32;
  let end_x = max_x.ceil() as i32;
  let end_y = max_y.ceil() as i32;

  for y in start_y..end_y {
    for x in start_x..end_x {
      // Transform once per pixel
      let point = Point {
        x: x as f32,
        y: y as f32,
      } * inverse;

      let canvas_x = x + offset.x;
      let canvas_y = y + offset.y;

      if canvas_x < 0 || canvas_y < 0 {
        continue;
      }

      let sampled_pixel = match algorithm {
        ImageScalingAlgorithm::Pixelated => interpolate_nearest(image, point.x, point.y),
        _ => interpolate_bilinear(image, point.x, point.y),
      };

      if let Some(pixel) = sampled_pixel {
        draw_pixel(canvas, canvas_x as u32, canvas_y as u32, pixel);
      }
    }
  }
}
