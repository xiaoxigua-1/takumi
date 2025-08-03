use image::{
  Rgba, RgbaImage,
  imageops::{fast_blur, overlay},
};
use taffy::{Layout, Point, Size};

use crate::{
  core::RenderContext,
  effects::{BorderRadius, apply_border_radius_antialiased},
  rendering::create_image_from_color_input,
  style::{BoxShadowInput, BoxShadowResolved},
};
use rayon::prelude::*;

use crate::rendering::FastBlendImage;

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Indicates which subset of box-shadows should be rendered for this pass.
pub enum BoxShadowRenderPhase {
  /// Render outer shadows (equivalent to `inset: false`)
  Outset,
  /// Render inner shadows (equivalent to `inset: true`)
  Inset,
}

/// Draws box shadows for an element, filtered by render phase (outset vs inset).
pub fn draw_box_shadow(
  context: &RenderContext,
  box_shadow_input: &BoxShadowInput,
  border_radius: Option<BorderRadius>,
  canvas: &mut FastBlendImage,
  layout: Layout,
  phase: BoxShadowRenderPhase,
) {
  match box_shadow_input {
    BoxShadowInput::Single(shadow) => {
      let resolved = shadow.clone().resolve(context);

      let matches_phase = match phase {
        BoxShadowRenderPhase::Outset => !resolved.inset,
        BoxShadowRenderPhase::Inset => resolved.inset,
      };

      if matches_phase {
        let draw = draw_single_box_shadow(&resolved, border_radius, layout);

        canvas.overlay_image(
          &draw.image,
          (layout.location.x + draw.offset.x) as u32,
          (layout.location.y + draw.offset.y) as u32,
        );
      }
    }
    BoxShadowInput::Multiple(shadows) => {
      // Preserve existing stacking order (reverse iteration) while filtering by phase.
      let images = shadows.par_iter().rev().filter_map(|shadow| {
        let resolved = shadow.clone().resolve(context);

        let matches_phase = match phase {
          BoxShadowRenderPhase::Outset => !resolved.inset,
          BoxShadowRenderPhase::Inset => resolved.inset,
        };

        if matches_phase {
          Some(draw_single_box_shadow(&resolved, border_radius, layout))
        } else {
          None
        }
      });

      for draw in images.collect::<Vec<_>>() {
        canvas.overlay_image(
          &draw.image,
          (layout.location.x + draw.offset.x) as u32,
          (layout.location.y + draw.offset.y) as u32,
        );
      }
    }
  }
}

struct ShadowDraw {
  image: RgbaImage,
  offset: Point<f32>,
}

fn draw_single_box_shadow(
  shadow: &BoxShadowResolved,
  border_radius: Option<BorderRadius>,
  layout: Layout,
) -> ShadowDraw {
  if shadow.inset {
    ShadowDraw {
      image: draw_inset_shadow(shadow, border_radius, layout),
      offset: Point { x: 0.0, y: 0.0 },
    }
  } else {
    ShadowDraw {
      image: draw_outset_shadow(shadow, border_radius, layout),
      offset: Point {
        x: shadow.offset_x - shadow.blur_radius - shadow.spread_radius,
        y: shadow.offset_y - shadow.blur_radius - shadow.spread_radius,
      },
    }
  }
}

fn draw_inset_shadow(
  shadow: &BoxShadowResolved,
  border_radius: Option<BorderRadius>,
  layout: Layout,
) -> RgbaImage {
  let mut shadow_image = create_image_from_color_input(
    &shadow.color,
    layout.size.width as u32,
    layout.size.height as u32,
  );

  remove_inner_section(
    &mut shadow_image,
    Point {
      x: (shadow.spread_radius + shadow.offset_x) as i32,
      y: (shadow.spread_radius + shadow.offset_y) as i32,
    },
    Size {
      width: (layout.size.width - shadow.spread_radius * 2.0) as u32,
      height: (layout.size.height - shadow.spread_radius * 2.0) as u32,
    },
    border_radius,
  );

  if shadow.blur_radius <= 0.0 {
    if let Some(border_radius) = border_radius {
      apply_border_radius_antialiased(&mut shadow_image, border_radius);
    }

    return shadow_image;
  }

  apply_fast_blur(&mut shadow_image, shadow.blur_radius);

  if let Some(border_radius) = border_radius {
    apply_border_radius_antialiased(&mut shadow_image, border_radius);
  }

  shadow_image
}

/// Draws an outset (external) box shadow.
fn draw_outset_shadow(
  shadow: &BoxShadowResolved,
  border_radius: Option<BorderRadius>,
  layout: Layout,
) -> RgbaImage {
  let mut spread_image = create_image_from_color_input(
    &shadow.color,
    (layout.size.width + shadow.spread_radius * 2.0) as u32,
    (layout.size.height + shadow.spread_radius * 2.0) as u32,
  );

  if let Some(mut border_radius) = border_radius {
    border_radius.offset_px(shadow.spread_radius);

    apply_border_radius_antialiased(&mut spread_image, border_radius);
  }

  if shadow.blur_radius <= 0.0 {
    return spread_image;
  }

  let box_shadow_size = (shadow.blur_radius + shadow.spread_radius) * 2.0;

  let mut blur_image = RgbaImage::new(
    (layout.size.width + box_shadow_size) as u32,
    (layout.size.height + box_shadow_size) as u32,
  );

  overlay(
    &mut blur_image,
    &spread_image,
    shadow.blur_radius as i64,
    shadow.blur_radius as i64,
  );

  apply_fast_blur(&mut blur_image, shadow.blur_radius);

  blur_image
}

fn remove_inner_section(
  image: &mut RgbaImage,
  offset: Point<i32>,
  size: Size<u32>,
  border_radius: Option<BorderRadius>,
) {
  let Some(border_radius) = border_radius else {
    let max_y = (offset.y + size.height as i32).min(image.height() as i32);
    let max_x = (offset.x + size.width as i32).min(image.width() as i32);

    image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
      if x as i32 >= offset.x && (x as i32) < max_x && y as i32 >= offset.y && (y as i32) < max_y {
        pixel.0[3] = 0;
      }
    });

    return;
  };

  let mut mask = RgbaImage::from_pixel(size.width, size.height, Rgba([0, 0, 0, 255]));

  apply_border_radius_antialiased(&mut mask, border_radius);

  image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
    let Some(masked_pixel) =
      mask.get_pixel_checked((x as i32 - offset.x) as u32, (y as i32 - offset.y) as u32)
    else {
      return;
    };

    match masked_pixel.0[3] {
      255 => {
        pixel.0[3] = 0;
      }
      0 => {}
      _ => {
        pixel.0[3] *= masked_pixel.0[3] / 255;
      }
    }
  });
}
