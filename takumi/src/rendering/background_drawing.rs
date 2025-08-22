use std::iter::successors;

use image::{
  Rgba, RgbaImage,
  imageops::{FilterType, resize},
};
use taffy::{Layout, Point, Size};

use crate::{
  layout::style::{
    BackgroundImage, BackgroundPosition, BackgroundRepeat, BackgroundRepeatStyle, BackgroundSize,
    Color, LengthUnit, PositionComponent, PositionKeywordX, PositionKeywordY, Style,
  },
  rendering::{BorderRadius, FastBlendImage, RenderContext},
};

/// Draws a filled rectangle with a solid color.
pub fn draw_filled_rect_color(
  canvas: &mut FastBlendImage,
  size: Size<f32>,
  offset: Point<f32>,
  color: Color,
  radius: Option<BorderRadius>,
) {
  let color: Rgba<u8> = color.into();
  let size = Size {
    width: size.width as u32,
    height: size.height as u32,
  };

  let Some(radius) = radius else {
    // Fast path: if drawing on the entire canvas, we can just replace the entire canvas with the color
    if color.0[3] == 255
      && offset.x == 0.0
      && offset.y == 0.0
      && size.width == canvas.width()
      && size.height == canvas.height()
    {
      let canvas_mut = canvas.0.as_mut();
      let canvas_len = canvas_mut.len();

      for i in (0..canvas_len).step_by(4) {
        canvas_mut[i..i + 4].copy_from_slice(&color.0);
      }

      return;
    }

    for y in 0..size.height {
      for x in 0..size.width {
        canvas.draw_pixel(x + offset.x as u32, y + offset.y as u32, color);
      }
    }

    return;
  };

  let mut image = RgbaImage::from_pixel(size.width, size.height, color);

  radius.apply_to_image(&mut image);

  canvas.overlay_image(&image, offset.x as u32, offset.y as u32);
}

fn resolve_length_against_area(unit: LengthUnit, area: u32, context: &RenderContext) -> u32 {
  if unit == LengthUnit::Auto {
    return area;
  }

  if let LengthUnit::Percentage(p) = unit {
    return ((p / 100.0) * area as f32).max(0.0) as u32;
  }

  unit.resolve_to_px(context).max(0.0) as u32
}

fn resolve_background_size(
  size: &BackgroundSize,
  area: (u32, u32),
  context: &RenderContext,
) -> (u32, u32) {
  match *size {
    BackgroundSize::Explicit { width, height } => (
      resolve_length_against_area(width, area.0, context),
      resolve_length_against_area(height, area.1, context),
    ),
    // as we only support gradients for now, we can just use the area size
    // if we want to support images, we need to resolve based on the image size
    _ => area,
  }
}

fn resolve_length_unit_to_position_component(
  length: LengthUnit,
  available: i32,
  context: &RenderContext,
) -> i32 {
  if length == LengthUnit::Auto {
    return available / 2;
  }

  if let LengthUnit::Percentage(p) = length {
    return ((available as f32) * (p / 100.0)).round() as i32;
  }

  length.resolve_to_px(context).round() as i32
}

fn resolve_position_component_x(
  comp: &BackgroundPosition,
  tile_w: u32,
  area_w: u32,
  context: &RenderContext,
) -> i32 {
  let available = area_w.saturating_sub(tile_w) as i32;
  match comp.x {
    PositionComponent::KeywordX(PositionKeywordX::Left) => 0,
    PositionComponent::KeywordX(PositionKeywordX::Center) => available / 2,
    PositionComponent::KeywordX(PositionKeywordX::Right) => available,
    PositionComponent::KeywordY(_) => available / 2,
    PositionComponent::Length(length) => {
      resolve_length_unit_to_position_component(length, available, context)
    }
  }
}

fn resolve_position_component_y(
  comp: &BackgroundPosition,
  tile_h: u32,
  area_h: u32,
  context: &RenderContext,
) -> i32 {
  let available = area_h.saturating_sub(tile_h) as i32;
  match comp.y {
    PositionComponent::KeywordY(PositionKeywordY::Top) => 0,
    PositionComponent::KeywordY(PositionKeywordY::Center) => available / 2,
    PositionComponent::KeywordY(PositionKeywordY::Bottom) => available,
    PositionComponent::KeywordX(_) => available / 2,
    PositionComponent::Length(length) => {
      resolve_length_unit_to_position_component(length, available, context)
    }
  }
}

/// Rasterize a single background image (gradient) into a tile of the given size.
/// resolving non-px stop units using the provided `RenderContext`.
pub fn render_gradient_tile(
  image: &BackgroundImage,
  tile_w: u32,
  tile_h: u32,
  context: &RenderContext,
) -> RgbaImage {
  match image {
    BackgroundImage::Linear(gradient) => {
      let mut ctx = gradient.to_draw_context(tile_w as f32, tile_h as f32, context);
      RgbaImage::from_fn(tile_w, tile_h, |x, y| gradient.at(x, y, &mut ctx).into())
    }
    BackgroundImage::Radial(gradient) => {
      let mut ctx = gradient.to_draw_context(tile_w as f32, tile_h as f32, context);
      RgbaImage::from_fn(tile_w, tile_h, |x, y| gradient.at(x, y, &mut ctx).into())
    }
  }
}

/// Collects a list of tile positions to place along an axis.
/// Starts from the "origin" and collects tile positions until the "area_size" is reached.
fn collect_repeat_tile_positions(area_size: u32, tile_size: u32, origin: i32) -> Vec<i32> {
  if tile_size == 0 {
    return Vec::new();
  }

  // Find first position, should be <= 0
  let mut start = origin;
  if start > 0 {
    let n = ((start as f32) / tile_size as f32).ceil() as i32;
    start -= n * tile_size as i32;
  }

  successors(Some(start), |&x| Some(x + tile_size as i32))
    .take_while(|&x| x < area_size as i32)
    .collect()
}

/// Collects evenly spaced tile positions along an axis for `background-repeat: space`.
/// Distributes gaps between tiles so the first and last touch the edges.
fn collect_spaced_tile_positions(area_size: u32, tile_size: u32) -> Vec<i32> {
  if tile_size == 0 {
    return vec![];
  }

  // Calculate number of tiles that fit in the area
  let count = area_size / tile_size;

  // Fast path: if there's only one tile, center it
  if count <= 1 {
    return vec![((area_size as i32 - tile_size as i32) / 2)];
  }

  // Calculate gap between tiles
  let gap = (area_size - count * tile_size) / (count - 1);
  let step = tile_size as i32 + gap as i32;

  successors(Some(0i32), move |&x| Some(x + step))
    .take(count as usize)
    .collect()
}

/// Collects stretched tile positions along an axis for `background-repeat: round`.
/// Rounds the size of the tile to fill the area.
/// Returns the positions and the new tile size.
fn collect_stretched_tile_positions(area_size: u32, tile_size: u32) -> (Vec<i32>, u32) {
  if tile_size == 0 || area_size == 0 {
    return (vec![], tile_size);
  }

  // Calculate number of tiles that fit in the area, at least 1
  let count = (area_size as f32 / tile_size as f32).floor().max(1.0) as u32;

  let new_tile_size = (area_size as f32 / count as f32).round() as u32;

  let positions = successors(Some(0i32), move |&x| Some(x + new_tile_size as i32))
    .take(count as usize)
    .collect();

  (positions, new_tile_size)
}

/// Pads a vector to `target_len` by repeating its last element, or `default` if empty.
fn pad_with_last<T: Copy>(values: &mut Vec<T>, target_len: usize, default: T) {
  let fill = values.last().copied().unwrap_or(default);
  values.resize(target_len, fill);
}

/// Draw layered backgrounds (gradients) with support for background-size, -position, and -repeat.
pub fn draw_background_layers(
  style: &Style,
  context: &RenderContext,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  let Some(images) = &style.background_image else {
    return;
  };

  let radius = style
    .inheritable_style
    .resolved_border_radius()
    .map(|r| BorderRadius::from_layout(context, &layout, r.into()));

  let area_w = layout.size.width as u32;
  let area_h = layout.size.height as u32;

  // Resolve per-layer lists with last-value semantics (repeat the last provided value)
  let mut positions = style
    .background_position
    .as_ref()
    .map(|v| v.0.clone())
    .unwrap_or_default();

  let mut sizes = style
    .background_size
    .as_ref()
    .map(|v| v.0.clone())
    .unwrap_or_default();

  let mut repeats = style
    .background_repeat
    .as_ref()
    .map(|v| v.0.clone())
    .unwrap_or_default();

  let target_len = images.0.len();
  pad_with_last(&mut positions, target_len, BackgroundPosition::default());
  pad_with_last(&mut sizes, target_len, BackgroundSize::default());
  pad_with_last(&mut repeats, target_len, BackgroundRepeat::repeat());

  // Paint each background layer in order
  for (i, image) in images.0.iter().enumerate() {
    let pos = &positions[i];
    let size = &sizes[i];
    let repeat = &repeats[i];

    // Compute tile size
    let (mut tile_w, mut tile_h) = resolve_background_size(size, (area_w, area_h), context);

    if tile_w == 0 || tile_h == 0 {
      continue;
    }

    // Build tile image (use context-aware resolver where possible)
    let mut tile_image = render_gradient_tile(image, tile_w, tile_h, context);

    // Handle round adjustment (rescale per axis)
    let xs: Vec<i32> = match repeat.x {
      BackgroundRepeatStyle::Repeat => {
        let origin_x = resolve_position_component_x(pos, tile_w, area_w, context);
        collect_repeat_tile_positions(area_w, tile_w, origin_x)
      }
      BackgroundRepeatStyle::NoRepeat => {
        let origin_x = resolve_position_component_x(pos, tile_w, area_w, context);
        vec![origin_x]
      }
      BackgroundRepeatStyle::Space => collect_spaced_tile_positions(area_w, tile_w),
      BackgroundRepeatStyle::Round => {
        let (px, new_w) = collect_stretched_tile_positions(area_w, tile_w);
        if new_w != tile_w {
          tile_w = new_w;
          tile_image = resize(&tile_image, tile_w, tile_h, FilterType::CatmullRom);
        }
        px
      }
    };

    let ys: Vec<i32> = match repeat.y {
      BackgroundRepeatStyle::Repeat => {
        let origin_y = resolve_position_component_y(pos, tile_h, area_h, context);
        collect_repeat_tile_positions(area_h, tile_h, origin_y)
      }
      BackgroundRepeatStyle::NoRepeat => {
        let origin_y = resolve_position_component_y(pos, tile_h, area_h, context);
        vec![origin_y]
      }
      BackgroundRepeatStyle::Space => collect_spaced_tile_positions(area_h, tile_h),
      BackgroundRepeatStyle::Round => {
        let (py, new_h) = collect_stretched_tile_positions(area_h, tile_h);
        if new_h != tile_h {
          tile_h = new_h;
          tile_image = resize(&tile_image, tile_w, tile_h, FilterType::CatmullRom);
        }
        py
      }
    };

    // Compose a layer-sized buffer
    let mut layer_img = FastBlendImage(RgbaImage::new(area_w, area_h));
    for y in &ys {
      for x in &xs {
        if *x >= area_w as i32 || *y >= area_h as i32 {
          continue;
        }

        layer_img.overlay_image(&tile_image, (*x).max(0) as u32, (*y).max(0) as u32);
      }
    }

    // Apply radius and overlay
    if let Some(r) = radius {
      r.apply_to_image(&mut layer_img.0);
    }

    canvas.overlay_image(
      &layer_img.0,
      layout.location.x as u32,
      layout.location.y as u32,
    );
  }
}
