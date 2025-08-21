use image::{
  Rgba, RgbaImage,
  imageops::{FilterType, overlay, resize},
};
use taffy::{Layout, Point, Size};

use crate::{
  layout::style::{
    BackgroundImage, BackgroundPosition, BackgroundRepeat, BackgroundRepeatStyle, BackgroundSize,
    Color, LengthUnit, LinearGradient, PositionComponent, PositionKeywordX, PositionKeywordY,
    RadialGradient, Style,
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

/// Draws a filled rectangle with a linear gradient.
pub fn draw_filled_rect_gradient(
  canvas: &mut FastBlendImage,
  size: Size<f32>,
  offset: Point<f32>,
  gradient: &LinearGradient,
  radius: Option<BorderRadius>,
) {
  let mut gradient_image = create_gradient_image(gradient, size.width as u32, size.height as u32);

  if let Some(radius) = radius {
    radius.apply_to_image(&mut gradient_image);
  }

  canvas.overlay_image(&gradient_image, offset.x as u32, offset.y as u32);
}

/// Creates an image from a gradient.
pub fn create_gradient_image(color: &LinearGradient, width: u32, height: u32) -> RgbaImage {
  let mut ctx = color.to_draw_context(width as f32, height as f32);
  RgbaImage::from_fn(width, height, |x, y| color.at(x, y, &mut ctx).into())
}

/// Draws a filled rectangle with a radial gradient.
pub fn draw_filled_rect_radial_gradient(
  canvas: &mut FastBlendImage,
  size: Size<f32>,
  offset: Point<f32>,
  gradient: &RadialGradient,
  radius: Option<BorderRadius>,
) {
  let mut gradient_image =
    create_radial_gradient_image(gradient, size.width as u32, size.height as u32);

  if let Some(radius) = radius {
    radius.apply_to_image(&mut gradient_image);
  }

  canvas.overlay_image(&gradient_image, offset.x as u32, offset.y as u32);
}

fn resolve_length_against_area(unit: LengthUnit, area: u32) -> u32 {
  match unit {
    LengthUnit::Auto => 0,
    LengthUnit::Px(v) => v.max(0.0) as u32,
    LengthUnit::Percentage(p) => ((p / 100.0) * area as f32).max(0.0) as u32,
    LengthUnit::Rem(_) | LengthUnit::Em(_) | LengthUnit::Vh(_) | LengthUnit::Vw(_) => {
      // Resolve these via pixels using viewport/parent font size isn't appropriate for background box.
      // Treat as pixels (already handled above) or fallback to zero.
      0
    }
  }
}

fn resolve_background_size(size: &BackgroundSize, area_w: u32, area_h: u32) -> (u32, u32) {
  match *size {
    BackgroundSize::Auto => (area_w, area_h),
    BackgroundSize::Cover | BackgroundSize::Contain => (area_w, area_h),
    BackgroundSize::Explicit { width, height } => {
      let w = match width {
        LengthUnit::Auto => area_w,
        _ => resolve_length_against_area(width, area_w),
      };
      let h = match height {
        LengthUnit::Auto => area_h,
        _ => resolve_length_against_area(height, area_h),
      };
      (w.max(1), h.max(1))
    }
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

fn zip_layers<T: Clone>(len: usize, list: &Option<Vec<T>>, default_item: &T) -> Vec<T> {
  let mut out = Vec::with_capacity(len);
  match list {
    Some(v) if !v.is_empty() => {
      for i in 0..len {
        out.push(
          v.get(i)
            .cloned()
            .unwrap_or_else(|| v.last().cloned().unwrap()),
        )
      }
    }
    _ => out.resize(len, default_item.clone()),
  }
  out
}

/// Rasterize a single background image (gradient) into a tile of the given size.
fn render_gradient_tile(image: &BackgroundImage, tile_w: u32, tile_h: u32) -> Option<RgbaImage> {
  match image {
    BackgroundImage::Linear(g) => Some(create_gradient_image(g, tile_w, tile_h)),
    BackgroundImage::Radial(g) => Some(create_radial_gradient_image(g, tile_w, tile_h)),
  }
}

fn place_tiles_axis_repeat(area: u32, tile: u32, origin: i32) -> Vec<i32> {
  if tile == 0 {
    return vec![];
  }
  let step = tile as i32;
  let mut positions = Vec::new();
  // Find first position within or before 0
  let mut start = origin;
  if start > 0 {
    let n = ((start as f32) / step as f32).ceil() as i32;
    start -= n * step;
  }
  while start < area as i32 {
    positions.push(start);
    start += step;
  }
  positions
}

fn place_tiles_axis_space(area: u32, tile: u32) -> (Vec<i32>, u32) {
  if tile == 0 {
    return (vec![], tile);
  }
  let n = area / tile;
  if n == 0 {
    // fallback to single centered
    return (vec![((area as i32 - tile as i32) / 2)], tile);
  }
  if n == 1 {
    return (vec![((area as i32 - tile as i32) / 2)], tile);
  }
  let gap = (area - n * tile) / (n - 1);
  let mut pos = Vec::with_capacity(n as usize);
  let mut x = 0i32;
  for _ in 0..n {
    pos.push(x);
    x += tile as i32 + gap as i32;
  }
  (pos, tile)
}

fn place_tiles_axis_round(area: u32, tile: u32) -> (Vec<i32>, u32) {
  if tile == 0 {
    return (vec![], tile);
  }
  let mut n = (area as f32 / tile as f32).round().max(1.0) as u32;
  if n == 0 {
    n = 1;
  }
  let new_tile = area / n;
  let mut pos = Vec::with_capacity(n as usize);
  let mut x = 0i32;
  for _ in 0..n {
    pos.push(x);
    x += new_tile as i32;
  }
  (pos, new_tile)
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

  // Resolve per-layer lists with last-value semantics
  let positions = style.background_position.as_ref().map(|v| v.0.clone());
  let sizes = style.background_size.as_ref().map(|v| v.0.clone());
  let repeats = style.background_repeat.as_ref().map(|v| v.0.clone());

  let pos_list = zip_layers(images.0.len(), &positions, &BackgroundPosition::default());
  let size_list = zip_layers(images.0.len(), &sizes, &BackgroundSize::Auto);
  let repeat_list = zip_layers(images.0.len(), &repeats, &BackgroundRepeat::repeat());

  // Paint each background layer in order
  for i in 0..images.0.len() {
    let image = &images.0[i];
    let pos = &pos_list[i];
    let size = &size_list[i];
    let repeat = &repeat_list[i];

    // Compute tile size
    let (mut tile_w, mut tile_h) = resolve_background_size(size, area_w, area_h);
    if tile_w == 0 || tile_h == 0 {
      continue;
    }

    // Build tile image
    let mut tile_image = match render_gradient_tile(image, tile_w, tile_h) {
      Some(img) => img,
      None => continue,
    };

    // Handle round adjustment (rescale per axis)
    let xs: Vec<i32> = match repeat.x {
      BackgroundRepeatStyle::Repeat => {
        let origin_x = resolve_position_component_x(pos, tile_w, area_w, context);
        place_tiles_axis_repeat(area_w, tile_w, origin_x)
      }
      BackgroundRepeatStyle::NoRepeat => {
        let origin_x = resolve_position_component_x(pos, tile_w, area_w, context);
        vec![origin_x]
      }
      BackgroundRepeatStyle::Space => {
        let (px, _) = place_tiles_axis_space(area_w, tile_w);
        px
      }
      BackgroundRepeatStyle::Round => {
        let (px, new_w) = place_tiles_axis_round(area_w, tile_w);
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
        place_tiles_axis_repeat(area_h, tile_h, origin_y)
      }
      BackgroundRepeatStyle::NoRepeat => {
        let origin_y = resolve_position_component_y(pos, tile_h, area_h, context);
        vec![origin_y]
      }
      BackgroundRepeatStyle::Space => {
        let (py, _) = place_tiles_axis_space(area_h, tile_h);
        py
      }
      BackgroundRepeatStyle::Round => {
        let (py, new_h) = place_tiles_axis_round(area_h, tile_h);
        if new_h != tile_h {
          tile_h = new_h;
          tile_image = resize(&tile_image, tile_w, tile_h, FilterType::CatmullRom);
        }
        py
      }
    };

    // Compose a layer-sized buffer
    let mut layer_img = RgbaImage::from_pixel(area_w, area_h, Rgba([0, 0, 0, 0]));
    for y in &ys {
      for x in &xs {
        let x = *x;
        let y = *y;

        if x >= area_w as i32 || y >= area_h as i32 {
          continue;
        }

        overlay(
          &mut layer_img,
          &tile_image,
          x.max(0) as i64,
          y.max(0) as i64,
        );
      }
    }

    // Apply radius and overlay
    if let Some(r) = radius {
      r.apply_to_image(&mut layer_img);
    }
    let x = layout.location.x as u32;
    let y = layout.location.y as u32;
    canvas.overlay_image(&layer_img, x, y);
  }
}

/// Creates an image from a radial gradient.
pub fn create_radial_gradient_image(
  gradient: &RadialGradient,
  width: u32,
  height: u32,
) -> RgbaImage {
  let mut ctx = gradient.to_draw_context(width as f32, height as f32);
  RgbaImage::from_fn(width, height, |x, y| gradient.at(x, y, &mut ctx).into())
}
