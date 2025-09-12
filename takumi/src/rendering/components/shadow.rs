use std::sync::Arc;

use image::{Rgba, RgbaImage, imageops::fast_blur};
use taffy::{Layout, Point, Size};
use zeno::{Fill, Mask};

use crate::{
  layout::style::{Affine, BoxShadow, BoxShadows, ImageScalingAlgorithm},
  rendering::{
    BorderProperties, Canvas, RenderContext, apply_mask_alpha_to_pixel, draw_filled_rect_color,
  },
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
pub(crate) enum BoxShadowRenderPhase {
  /// Render outer shadows (equivalent to `inset: false`)
  Outset,
  /// Render inner shadows (equivalent to `inset: true`)
  Inset,
}

/// Represents a resolved box shadow with all its properties.
pub(crate) struct BoxShadowResolved {
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
  pub fn from_box_shadow(shadow: &BoxShadow, context: &RenderContext, size: Size<f32>) -> Self {
    Self {
      inset: shadow.inset,
      offset_x: shadow.offset_x.resolve_to_px(context, size.width),
      offset_y: shadow.offset_y.resolve_to_px(context, size.height),
      blur_radius: shadow.blur_radius.resolve_to_px(context, size.width),
      spread_radius: shadow
        .spread_radius
        .resolve_to_px(context, size.width)
        .max(0.0),
      color: shadow.color.into(),
    }
  }
}

/// Draws box shadows for an element, filtered by render phase (outset vs inset).
pub(crate) fn draw_box_shadow(
  context: &RenderContext,
  box_shadows: &BoxShadows,
  border_radius: BorderProperties,
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
        let resolved = BoxShadowResolved::from_box_shadow(shadow, context, layout.size);

        let draw = draw_single_box_shadow(&resolved, border_radius, layout);

        canvas.overlay_image(
          Arc::new(draw.image),
          Point {
            x: (layout.location.x + draw.offset.x) as i32,
            y: (layout.location.y + draw.offset.y) as i32,
          },
          draw.border_radius,
          context.transform,
          ImageScalingAlgorithm::Auto,
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
          Some(BoxShadowResolved::from_box_shadow(
            shadow,
            context,
            layout.size,
          ))
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
          Arc::new(draw.image),
          Point {
            x: (layout.location.x + draw.offset.x) as i32,
            y: (layout.location.y + draw.offset.y) as i32,
          },
          draw.border_radius,
          context.transform,
          ImageScalingAlgorithm::Auto,
        );
      }
    }
  }
}

struct ShadowDraw {
  image: RgbaImage,
  offset: Point<f32>,
  border_radius: BorderProperties,
}

fn draw_single_box_shadow(
  shadow: &BoxShadowResolved,
  border: BorderProperties,
  layout: Layout,
) -> ShadowDraw {
  if shadow.inset {
    ShadowDraw {
      image: draw_inset_shadow(shadow, border, layout),
      offset: Point { x: 0.0, y: 0.0 },
      border_radius: border,
    }
  } else {
    ShadowDraw {
      image: draw_outset_shadow(shadow, border, layout),
      offset: Point {
        x: shadow.offset_x - shadow.blur_radius - shadow.spread_radius,
        y: shadow.offset_y - shadow.blur_radius - shadow.spread_radius,
      },
      border_radius: BorderProperties::zero(),
    }
  }
}

fn draw_inset_shadow(
  shadow: &BoxShadowResolved,
  border: BorderProperties,
  layout: Layout,
) -> RgbaImage {
  let mut shadow_image = RgbaImage::from_pixel(
    layout.size.width as u32,
    layout.size.height as u32,
    shadow.color,
  );

  let mut paths = Vec::new();

  let border = BorderProperties {
    offset: Point {
      x: shadow.offset_x,
      y: shadow.offset_y,
    },
    ..border
  };

  border.append_mask_commands(&mut paths);

  border
    .expand_by(-shadow.spread_radius)
    .append_mask_commands(&mut paths);

  let (mask, placement) = Mask::new(&paths).style(Fill::EvenOdd).render();

  let mut i = 0;

  for y in 0..placement.height {
    for x in 0..placement.width {
      let alpha = mask[i];

      i += 1;

      if alpha == u8::MAX {
        continue;
      }

      let x = x as i32 + placement.left;
      let y = y as i32 + placement.top;

      let color = apply_mask_alpha_to_pixel(shadow.color.0.into(), alpha);

      if let Some(pixel) = shadow_image.get_pixel_mut_checked(x as u32, y as u32) {
        *pixel = color;
      }
    }
  }

  apply_fast_blur(&mut shadow_image, shadow.blur_radius);

  shadow_image
}

/// Draws an outset box shadow.
fn draw_outset_shadow(
  shadow: &BoxShadowResolved,
  border: BorderProperties,
  layout: Layout,
) -> RgbaImage {
  let box_shadow_size = (shadow.blur_radius + shadow.spread_radius) * 2.0;

  let mut image = RgbaImage::new(
    (layout.size.width + box_shadow_size) as u32,
    (layout.size.height + box_shadow_size) as u32,
  );

  // Draw the spread area, offset by blur + spread radius, width is spread radius
  draw_filled_rect_color(
    &mut image,
    Size {
      width: layout.size.width as u32,
      height: layout.size.height as u32,
    },
    Point {
      x: (shadow.spread_radius + shadow.blur_radius) as i32,
      y: (shadow.spread_radius + shadow.blur_radius) as i32,
    },
    shadow.color,
    border.expand_by(shadow.spread_radius),
    Affine::identity(),
  );

  if shadow.blur_radius <= 0.0 {
    return image;
  }

  apply_fast_blur(&mut image, shadow.blur_radius);

  image
}
