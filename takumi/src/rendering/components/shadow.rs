use image::{Rgba, RgbaImage, imageops::fast_blur};
use taffy::{Layout, Point, Size};

use crate::{
  layout::style::{BoxShadow, BoxShadows},
  rendering::{BorderRadius, Canvas, RenderContext},
};

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

/// Represents a resolved box shadow with all its properties.
pub struct BoxShadowResolved {
  /// Whether the shadow is inset or outset.
  pub inset: bool,
  /// Horizontal offset of the shadow.
  pub offset_x: f32,
  /// Vertical offset of the shadow.
  pub offset_y: f32,
  /// Blur radius of the shadow. Higher values create a more blurred shadow.
  pub blur_radius: f32,
  /// Spread radius of the shadow. Positive values expand the shadow, negative values shrink it.
  pub spread_radius: f32,
  /// Color of the shadow.
  pub color: Rgba<u8>,
}

impl BoxShadowResolved {
  /// Creates a new `BoxShadowResolved` from a `BoxShadow` and a `RenderContext`.
  pub fn from_box_shadow(shadow: &BoxShadow, context: &RenderContext) -> Self {
    Self {
      inset: shadow.inset,
      offset_x: shadow.offset_x.resolve_to_px(context),
      offset_y: shadow.offset_y.resolve_to_px(context),
      blur_radius: shadow.blur_radius.resolve_to_px(context),
      spread_radius: shadow.spread_radius.resolve_to_px(context),
      color: shadow.color.into(),
    }
  }
}

/// Draws box shadows for an element, filtered by render phase (outset vs inset).
pub fn draw_box_shadow(
  context: &RenderContext,
  box_shadows: &BoxShadows,
  border_radius: BorderRadius,
  canvas: &Canvas,
  layout: Layout,
  phase: BoxShadowRenderPhase,
) {
  match &box_shadows.0[..] {
    [shadow] => {
      let matches_phase = match phase {
        BoxShadowRenderPhase::Outset => !shadow.inset,
        BoxShadowRenderPhase::Inset => shadow.inset,
      };

      if matches_phase {
        let resolved = BoxShadowResolved::from_box_shadow(shadow, context);

        let draw = draw_single_box_shadow(&resolved, border_radius, layout);

        canvas.overlay_image(
          draw.image,
          Point {
            x: (layout.location.x + draw.offset.x) as i32,
            y: (layout.location.y + draw.offset.y) as i32,
          },
          border_radius,
        );
      }
    }
    shadows => {
      let to_draw = shadows.iter().filter_map(|shadow| {
        let matches_phase = match phase {
          BoxShadowRenderPhase::Outset => !shadow.inset,
          BoxShadowRenderPhase::Inset => shadow.inset,
        };

        if matches_phase {
          Some(BoxShadowResolved::from_box_shadow(shadow, context))
        } else {
          None
        }
      });

      // Preserve existing stacking order (reverse iteration) while filtering by phase.
      #[cfg(feature = "rayon")]
      let images = {
        use rayon::iter::{ParallelBridge, ParallelIterator};

        to_draw
          .rev()
          .par_bridge()
          .map(|shadow| draw_single_box_shadow(&shadow, border_radius, layout))
          .collect::<Vec<_>>()
      };

      #[cfg(not(feature = "rayon"))]
      let images = to_draw
        .rev()
        .map(|shadow| draw_single_box_shadow(&shadow, border_radius, layout))
        .collect::<Vec<_>>();

      for draw in images {
        canvas.overlay_image(
          draw.image,
          Point {
            x: (layout.location.x + draw.offset.x) as i32,
            y: (layout.location.y + draw.offset.y) as i32,
          },
          border_radius,
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
  border_radius: BorderRadius,
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
  border_radius: BorderRadius,
  layout: Layout,
) -> RgbaImage {
  let mut shadow_image = RgbaImage::from_pixel(
    layout.size.width as u32,
    layout.size.height as u32,
    shadow.color,
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

  apply_fast_blur(&mut shadow_image, shadow.blur_radius);

  shadow_image
}

/// Draws an outset (external) box shadow.
fn draw_outset_shadow(
  shadow: &BoxShadowResolved,
  mut border_radius: BorderRadius,
  layout: Layout,
) -> RgbaImage {
  let mut spread_image = RgbaImage::from_pixel(
    (layout.size.width + shadow.spread_radius * 2.0) as u32,
    (layout.size.height + shadow.spread_radius * 2.0) as u32,
    shadow.color,
  );

  if !border_radius.is_zero() {
    border_radius.offset_px(shadow.spread_radius);
    border_radius.apply_to_image(&mut spread_image);
  }

  if shadow.blur_radius <= 0.0 {
    return spread_image;
  }

  let box_shadow_size = (shadow.blur_radius + shadow.spread_radius) * 2.0;

  let mut blur_image = FastBlendImage(RgbaImage::new(
    (layout.size.width + box_shadow_size) as u32,
    (layout.size.height + box_shadow_size) as u32,
  ));

  blur_image.overlay_image(
    &spread_image,
    shadow.blur_radius as i32,
    shadow.blur_radius as i32,
  );

  apply_fast_blur(&mut blur_image.0, shadow.blur_radius);

  blur_image.0
}

fn get_pixel_index_from_axis(x: u32, y: u32, width: u32) -> usize {
  (y * width + x) as usize
}

fn remove_inner_section(
  image: &mut RgbaImage,
  offset: Point<i32>,
  size: Size<u32>,
  border_radius: BorderRadius,
) {
  if border_radius.is_zero() {
    let width = image.width();
    let image_mut = image.as_mut();

    for x in 0..size.width {
      for y in 0..size.height {
        let index = get_pixel_index_from_axis(x + offset.x as u32, y + offset.y as u32, width);

        image_mut[index * 4 + 3] = 0;
      }
    }

    return;
  };

  let mut mask = RgbaImage::from_pixel(size.width, size.height, Rgba([0, 0, 0, 255]));

  border_radius.apply_to_image(&mut mask);

  let width = image.width();
  let pixels = image.as_mut();

  for (x, y, mask_pixel) in mask.enumerate_pixels() {
    match mask_pixel.0[3] {
      255 => {
        let index = get_pixel_index_from_axis(x + offset.x as u32, y + offset.y as u32, width);

        pixels[index * 4 + 3] = 0;
      }
      0 => {}
      _ => {
        let index = get_pixel_index_from_axis(x + offset.x as u32, y + offset.y as u32, width);

        pixels[index * 4 + 3] =
          (pixels[index * 4 + 3] as f32 * (mask_pixel.0[3] as f32 / 255.0)) as u8;
      }
    }
  }
}
