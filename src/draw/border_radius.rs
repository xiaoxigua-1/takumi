use image::RgbaImage;

pub fn apply_border_radius_optimized(img: &mut RgbaImage, radius: u32) {
  let (width, height) = img.dimensions();
  let radius_sq = radius * radius;

  // Process only corner regions - skip the center entirely
  process_corner_region(
    img,
    0,
    0,
    radius,
    radius,
    radius,
    radius_sq,
    Corner::TopLeft,
  );
  process_corner_region(
    img,
    width - radius,
    0,
    width,
    radius,
    radius,
    radius_sq,
    Corner::TopRight,
  );
  process_corner_region(
    img,
    0,
    height - radius,
    radius,
    height,
    radius,
    radius_sq,
    Corner::BottomLeft,
  );
  process_corner_region(
    img,
    width - radius,
    height - radius,
    width,
    height,
    radius,
    radius_sq,
    Corner::BottomRight,
  );
}

#[derive(Copy, Clone)]
enum Corner {
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
}

#[inline]
fn process_corner_region(
  img: &mut RgbaImage,
  start_x: u32,
  start_y: u32,
  end_x: u32,
  end_y: u32,
  radius: u32,
  radius_sq: u32,
  corner: Corner,
) {
  let (corner_x, corner_y) = match corner {
    Corner::TopLeft => (radius - 1, radius - 1),
    Corner::TopRight => (start_x, radius - 1),
    Corner::BottomLeft => (radius - 1, start_y),
    Corner::BottomRight => (start_x, start_y),
  };

  // Process in chunks for better cache locality
  for y in start_y..end_y {
    let dy = if y > corner_y {
      y - corner_y
    } else {
      corner_y - y
    };
    let dy_sq = dy * dy;

    // Early exit if entire row is outside radius
    if dy_sq > radius_sq {
      // Set entire row to transparent
      let row_start = ((y * img.width() + start_x) * 4 + 3) as usize;
      let pixels = img.as_mut();
      for i in 0..(end_x - start_x) {
        pixels[row_start + (i * 4) as usize] = 0;
      }
      continue;
    }

    for x in start_x..end_x {
      let dx = if x > corner_x {
        x - corner_x
      } else {
        corner_x - x
      };
      let dist_sq = dx * dx + dy_sq; // Reuse dy_sq

      if dist_sq > radius_sq {
        let idx = ((y * img.width() + x) * 4 + 3) as usize;
        img.as_mut()[idx] = 0;
      }
    }
  }
}
