use image::RgbaImage;
use taffy::Point;

/// Applies antialiased border radius to an image.
///
/// This function processes the corners of the image to create smooth, antialiased rounded corners.
/// The border radius is automatically clamped to half the minimum dimension of the image.
///
/// # Arguments
/// * `img` - The image to apply border radius to
/// * `radius` - The radius of the corners in pixels
pub fn apply_border_radius_antialiased(img: &mut RgbaImage, mut radius: f32) {
  let (width, height) = img.dimensions();

  radius = radius.min(width.min(height) as f32 / 2.0);

  let transition_width = 1.0;
  let outer_radius = radius + transition_width;
  let outer_radius_sq = outer_radius * outer_radius;
  let radius_sq = radius * radius;

  // Process only corner regions with antialiasing band
  let band_size =
    (outer_radius.ceil() as u32).max(radius as u32 + (transition_width * 2.0).ceil() as u32);

  let corners = [
    (
      Point { x: 0, y: 0 },
      Point {
        x: band_size,
        y: band_size,
      },
      Corner::TopLeft,
    ),
    (
      Point {
        x: width.saturating_sub(band_size),
        y: 0,
      },
      Point {
        x: width,
        y: band_size,
      },
      Corner::TopRight,
    ),
    (
      Point {
        x: 0,
        y: height.saturating_sub(band_size),
      },
      Point {
        x: band_size,
        y: height,
      },
      Corner::BottomLeft,
    ),
    (
      Point {
        x: width.saturating_sub(band_size),
        y: height.saturating_sub(band_size),
      },
      Point {
        x: width,
        y: height,
      },
      Corner::BottomRight,
    ),
  ];

  for (start, end, corner) in corners {
    process_corner_aa(img, start, end, radius, radius_sq, outer_radius_sq, corner);
  }
}

/// Represents the four corners of an image for border radius processing.
#[derive(Copy, Clone)]
enum Corner {
  /// Top-left corner
  TopLeft,
  /// Top-right corner
  TopRight,
  /// Bottom-left corner
  BottomLeft,
  /// Bottom-right corner
  BottomRight,
}

/// Processes a single corner of the image for antialiased border radius.
///
/// This function handles the antialiasing calculations for a specific corner,
/// creating a smooth transition between the rounded corner and the rest of the image.
///
/// # Arguments
/// * `img` - The image being processed
/// * `start` - Starting point of the corner region
/// * `end` - Ending point of the corner region
/// * `radius` - The radius of the corner
/// * `radius_sq` - The squared radius (for optimization)
/// * `outer_radius_sq` - The squared outer radius of the antialiasing band
/// * `corner` - Which corner is being processed
#[inline]
fn process_corner_aa(
  img: &mut RgbaImage,
  start: Point<u32>,
  end: Point<u32>,
  radius: f32,
  radius_sq: f32,
  outer_radius_sq: f32,
  corner: Corner,
) {
  let (corner_x, corner_y) = match corner {
    Corner::TopLeft => (radius, radius),
    Corner::TopRight => (start.x as f32, radius),
    Corner::BottomLeft => (radius, start.y as f32),
    Corner::BottomRight => (start.x as f32, start.y as f32),
  };

  for y in start.y..end.y {
    let fy = y as f32;
    let dy = (fy - corner_y).abs();
    let dy_sq = dy * dy;

    // Early exit optimization - if entire row is outside outer radius
    if dy_sq > outer_radius_sq {
      set_row_alpha(img, start.x, end.x, y, 0);
      continue;
    }

    // Early exit - if entire row is inside radius
    if dy_sq < radius_sq
      && (start.x as f32 - corner_x).abs() < radius
      && (end.x as f32 - corner_x).abs() < radius
    {
      continue; // Keep original alpha
    }

    for x in start.x..end.x {
      let fx = x as f32;
      let dx = (fx - corner_x).abs();
      let dist_sq = dx * dx + dy_sq;

      let alpha = if dist_sq <= radius_sq {
        255 // Inside radius - keep original
      } else if dist_sq >= outer_radius_sq {
        0 // Outside antialiasing band - transparent
      } else {
        // Antialiasing zone - smooth transition
        let dist = dist_sq.sqrt();
        let factor = (outer_radius_sq.sqrt() - dist) / (outer_radius_sq.sqrt() - radius_sq.sqrt());
        (factor * 255.0).clamp(0.0, 255.0) as u8
      };

      if alpha < 255 {
        let idx = ((y * img.width() + x) * 4 + 3) as usize;
        let pixels = img.as_mut();
        if alpha == 0 {
          pixels[idx] = 0;
        } else {
          // Blend with existing alpha
          let existing = pixels[idx] as u32;
          pixels[idx] = ((existing * alpha as u32) / 255) as u8;
        }
      }
    }
  }
}

/// Sets the alpha value for an entire row of pixels in the image.
///
/// # Arguments
/// * `img` - The image to modify
/// * `start_x` - Starting x coordinate
/// * `end_x` - Ending x coordinate
/// * `y` - The y coordinate of the row
/// * `alpha` - The alpha value to set (0-255)
#[inline]
fn set_row_alpha(img: &mut RgbaImage, start_x: u32, end_x: u32, y: u32, alpha: u8) {
  let width = img.width();
  let pixels = img.as_mut();
  for x in start_x..end_x {
    let idx = ((y * width + x) * 4 + 3) as usize;
    pixels[idx] = alpha;
  }
}
