use std::sync::Arc;
use std::{borrow::Cow, collections::HashMap};

use image::RgbaImage;
use parley::{Glyph, PositionedLayoutItem, StyleProperty};
#[cfg(feature = "rayon")]
use rayon::iter::{ParallelBridge, ParallelIterator};
use swash::{
  Setting,
  scale::{Scaler, StrikeWith, image::Image, outline::Outline},
  tag_from_bytes,
};
use taffy::{Layout, Point, Size};
use zeno::{Command, Mask, PathData, Placement};

use crate::{
  GlobalContext,
  layout::style::{
    Affine, Color, ImageScalingAlgorithm, ResolvedFontStyle, Style, TextOverflow, TextTransform,
  },
  rendering::{
    BorderProperties, Canvas, RenderContext, apply_mask_alpha_to_pixel, overlay_image,
    resolve_layers_tiles,
  },
};

const ELLIPSIS_CHAR: &str = "…";

/// Draws text on the canvas with the specified font style and layout.
pub fn draw_text(
  text: &str,
  style: &Style,
  context: &RenderContext,
  canvas: &Canvas,
  layout: Layout,
) {
  let font_style = style.resolve_to_font_style(context);
  if font_style.font_size == 0.0 {
    return;
  }

  let content_box = layout.content_box_size();

  let render_text = apply_text_transform(text, font_style.text_transform);

  let mut buffer = create_text_layout(
    &render_text,
    &font_style,
    context.global,
    content_box.width,
    Some(MaxHeight::Absolute(content_box.height)),
  );

  let Some(last_line) = buffer.lines().last() else {
    return;
  };

  let last_line_range = last_line.text_range();

  let should_append_ellipsis =
    font_style.text_overflow == TextOverflow::Ellipsis && last_line_range.end < render_text.len();

  if should_append_ellipsis {
    let text_with_ellipsis = make_ellipsis_text(
      &render_text,
      last_line_range.start,
      last_line_range.end,
      &font_style,
      context.global,
      content_box.width,
    );

    buffer = create_text_layout(
      &text_with_ellipsis,
      &font_style,
      context.global,
      content_box.width,
      Some(MaxHeight::Absolute(content_box.height)),
    );
  }

  // If we have a mask image on the style, render it using the background tiling logic into a
  // temporary image and use that as the glyph fill.
  if let Some(images) = &style.mask_image {
    let resolved_tiles = resolve_layers_tiles(
      images,
      style.mask_position.as_ref(),
      style.mask_size.as_ref(),
      style.mask_repeat.as_ref(),
      context,
      layout,
    );

    if resolved_tiles.is_empty() {
      return;
    }

    let mut composed = RgbaImage::new(content_box.width as u32, content_box.height as u32);

    for (tile_image, xs, ys) in resolved_tiles {
      for y in &ys {
        for x in &xs {
          overlay_image(
            &mut composed,
            &tile_image,
            Point { x: *x, y: *y },
            Default::default(),
            Affine::identity(),
            ImageScalingAlgorithm::Auto,
          )
        }
      }
    }

    draw_buffer(
      context,
      &buffer,
      canvas,
      style.inheritable_style.color.unwrap_or_else(Color::black),
      layout,
      Some(composed),
    );

    return;
  }

  draw_buffer(
    context,
    &buffer,
    canvas,
    style.inheritable_style.color.unwrap_or_else(Color::black),
    layout,
    None,
  );
}

fn draw_buffer(
  context: &RenderContext,
  buffer: &parley::Layout<()>,
  canvas: &Canvas,
  color: Color,
  layout: Layout,
  image_fill: Option<RgbaImage>,
) {
  for line in buffer.lines() {
    for item in line.items() {
      let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
        continue;
      };

      let run = glyph_run.run();

      let resolved_glyphs = context.global.font_context.with_scaler(run, |scaler| {
        let mut unique_glyph_ids = glyph_run.glyphs().map(|glyph| glyph.id).collect::<Vec<_>>();

        unique_glyph_ids.sort_unstable();
        unique_glyph_ids.dedup();

        unique_glyph_ids
          .iter()
          .filter_map(|glyph_id| Some((*glyph_id, Arc::new(resolve_glyph(*glyph_id, scaler)?))))
          .collect::<HashMap<u16, Arc<ResolvedGlyph>>>()
      });

      #[cfg(feature = "rayon")]
      {
        glyph_run
          .positioned_glyphs()
          .filter_map(|glyph| Some((glyph, resolved_glyphs.get(&glyph.id)?.clone())))
          .par_bridge()
          .for_each(|(glyph, resolved_glyph)| {
            draw_glyph(
              glyph,
              &resolved_glyph,
              canvas,
              color,
              layout,
              image_fill.as_ref(),
              context.transform,
            );
          });
      }

      #[cfg(not(feature = "rayon"))]
      {
        glyph_run
          .positioned_glyphs()
          .filter_map(|glyph| Some((glyph, resolved_glyphs.get(&glyph.id)?.clone())))
          .for_each(|(glyph, resolved_glyph)| {
            draw_glyph(
              glyph,
              &resolved_glyph,
              canvas,
              color,
              layout,
              image_fill.as_ref(),
              context.transform,
            );
          });
      }
    }
  }
}

enum ResolvedGlyph {
  Outline(Outline),
  Image(Image),
}

fn resolve_glyph(glyph_id: u16, scaler: &mut Scaler<'_>) -> Option<ResolvedGlyph> {
  if let Some(bitmap) = scaler.scale_color_bitmap(glyph_id, StrikeWith::BestFit) {
    return Some(ResolvedGlyph::Image(bitmap));
  }

  if let Some(outline) = scaler.scale_color_outline(glyph_id) {
    return Some(ResolvedGlyph::Outline(outline));
  }

  if let Some(outline) = scaler.scale_outline(glyph_id) {
    return Some(ResolvedGlyph::Outline(outline));
  }

  None
}

fn draw_glyph(
  glyph: Glyph,
  resolved_glyph: &ResolvedGlyph,
  canvas: &Canvas,
  color: Color,
  layout: Layout,
  image_fill: Option<&RgbaImage>,
  transform: Affine,
) {
  let transform = Affine::translation(Size {
    width: layout.border.left + layout.padding.left + glyph.x,
    height: layout.border.top + layout.padding.top + glyph.y,
  }) * transform;

  if let ResolvedGlyph::Image(bitmap) = resolved_glyph {
    let border = BorderProperties {
      size: Size {
        width: bitmap.placement.width as f32,
        height: bitmap.placement.height as f32,
      },
      ..Default::default()
    };

    let offset = Point {
      x: layout.location.x as i32 + bitmap.placement.left,
      y: layout.location.y as i32 - bitmap.placement.top,
    };

    if let Some(image_fill) = image_fill {
      let mask = bitmap
        .data
        .iter()
        .skip(3)
        .step_by(4)
        .copied()
        .collect::<Vec<_>>();

      let placement = Placement {
        left: 0,
        top: 0,
        width: bitmap.placement.width,
        height: bitmap.placement.height,
      };

      let mut bottom = RgbaImage::new(placement.width, placement.height);

      let mut i = 0;

      for y in 0..placement.height {
        for x in 0..placement.width {
          let alpha = mask[i];
          i += 1;

          if alpha == 0 {
            continue;
          }

          let source_x = x + glyph.x as u32;
          let source_y = y + glyph.y as u32 - bitmap.placement.top as u32;

          let Some(pixel) = image_fill.get_pixel_checked(source_x, source_y) else {
            continue;
          };

          let pixel = apply_mask_alpha_to_pixel(*pixel, alpha);

          bottom.put_pixel(x, y, pixel);
        }
      }

      return canvas.overlay_image(
        Arc::new(bottom),
        offset,
        border,
        transform,
        ImageScalingAlgorithm::Auto,
      );
    }

    let image = RgbaImage::from_raw(
      bitmap.placement.width,
      bitmap.placement.height,
      bitmap.data.clone(),
    )
    .unwrap();

    return canvas.overlay_image(
      Arc::new(image),
      offset,
      border,
      transform,
      ImageScalingAlgorithm::Auto,
    );
  }

  if let ResolvedGlyph::Outline(outline) = resolved_glyph {
    // have to invert the y coordinate from y-up to y-down first
    let mut paths = outline
      .path()
      .commands()
      .map(|command| match command {
        Command::MoveTo(point) => Command::MoveTo((point.x, -point.y).into()),
        Command::LineTo(point) => Command::LineTo((point.x, -point.y).into()),
        Command::CurveTo(point1, point2, point3) => Command::CurveTo(
          (point1.x, -point1.y).into(),
          (point2.x, -point2.y).into(),
          (point3.x, -point3.y).into(),
        ),
        Command::QuadTo(point1, point2) => {
          Command::QuadTo((point1.x, -point1.y).into(), (point2.x, -point2.y).into())
        }
        Command::Close => Command::Close,
      })
      .collect::<Vec<_>>();

    transform.apply_on_paths(&mut paths);

    let (mask, mut placement) = Mask::new(&paths).render();

    let cropped_fill_image = image_fill.map(|image| {
      let mut bottom = RgbaImage::new(placement.width, placement.height);

      for y in 0..placement.height {
        let dest_y = y + placement.top as u32;

        if dest_y >= image.height() {
          continue;
        }

        for x in 0..placement.width {
          let dest_x = x + placement.left as u32;

          if dest_x >= image.width() {
            continue;
          }

          bottom.put_pixel(x, y, *image.get_pixel(dest_x, dest_y));
        }
      }

      bottom
    });

    placement.left += layout.location.x as i32;
    placement.top += layout.location.y as i32;

    canvas.draw_mask(mask, placement, color, cropped_fill_image);
  }
}

pub(crate) enum MaxHeight {
  Absolute(f32),
  Lines(u32),
  Both(f32, u32),
}

const VARIABLE_FONT_WEIGHT_TAG: u32 = tag_from_bytes(b"wght");

pub(crate) fn create_text_layout(
  text: &str,
  font_style: &ResolvedFontStyle,
  global: &GlobalContext,
  max_width: f32,
  max_height: Option<MaxHeight>,
) -> parley::Layout<()> {
  let mut layout = global.font_context.create_layout(text, |builder| {
    builder.push_default(StyleProperty::FontSize(font_style.font_size));
    builder.push_default(StyleProperty::LineHeight(font_style.line_height));
    builder.push_default(StyleProperty::FontWeight(font_style.font_weight));
    builder.push_default(StyleProperty::FontStyle(font_style.font_style));

    if let Some(font_variation_settings) = font_style.font_variation_settings.as_ref()
      && !font_variation_settings.0.is_empty()
    {
      builder.push_default(StyleProperty::FontVariations(parley::FontSettings::List(
        Cow::Borrowed(&font_variation_settings.0),
      )));
    } else {
      let variable_font_setting = Setting {
        tag: VARIABLE_FONT_WEIGHT_TAG,
        value: font_style.font_weight.value(),
      };

      builder.push_default(StyleProperty::FontVariations(parley::FontSettings::List(
        Cow::Borrowed(&[variable_font_setting]),
      )));
    }

    if let Some(font_feature_settings) = font_style.font_feature_settings.as_ref()
      && !font_feature_settings.0.is_empty()
    {
      builder.push_default(StyleProperty::FontFeatures(parley::FontSettings::List(
        Cow::Borrowed(&font_feature_settings.0),
      )));
    }

    if let Some(font_family) = font_style.font_family.as_ref() {
      builder.push_default(StyleProperty::FontStack(font_family.into()));
    }

    if let Some(letter_spacing) = font_style.letter_spacing {
      builder.push_default(StyleProperty::LetterSpacing(letter_spacing));
    }

    if let Some(word_spacing) = font_style.word_spacing {
      builder.push_default(StyleProperty::WordSpacing(word_spacing));
    }

    builder.push_default(StyleProperty::WordBreak(font_style.word_break));
    builder.push_default(StyleProperty::OverflowWrap(font_style.overflow_wrap));
  });

  break_lines(&mut layout, max_width, max_height);

  layout.align(
    Some(max_width),
    font_style.text_align.unwrap_or_default(),
    Default::default(),
  );

  layout
}

fn break_lines(layout: &mut parley::Layout<()>, max_width: f32, max_height: Option<MaxHeight>) {
  let Some(max_height) = max_height else {
    return layout.break_all_lines(Some(max_width));
  };

  match max_height {
    MaxHeight::Lines(lines) => {
      let mut breaker = layout.break_lines();

      for _ in 0..lines {
        if breaker.break_next(max_width).is_none() {
          // no more lines to break
          break;
        };
      }

      breaker.finish();
    }
    MaxHeight::Absolute(max_height) => {
      let mut total_height = 0.0;
      let mut breaker = layout.break_lines();

      while total_height < max_height {
        let Some((_, height)) = breaker.break_next(max_width) else {
          // no more lines to break
          break;
        };

        total_height += height;
      }

      // if its over the max height after last break, revert the break
      if total_height > max_height {
        breaker.revert();
      }

      breaker.finish();
    }
    MaxHeight::Both(max_height, max_lines) => {
      let mut total_height = 0.0;
      let mut line_count = 0;
      let mut breaker = layout.break_lines();

      while total_height < max_height {
        if line_count >= max_lines {
          break;
        }

        let Some((_, height)) = breaker.break_next(max_width) else {
          // no more lines to break
          break;
        };

        line_count += 1;
        total_height += height;
      }

      if total_height > max_height {
        breaker.revert();
      }

      breaker.finish();
    }
  }
}

/// Applies text transform to the input text.
pub fn apply_text_transform<'a>(input: &'a str, transform: TextTransform) -> Cow<'a, str> {
  match transform {
    TextTransform::None => Cow::Borrowed(input),
    TextTransform::Uppercase => Cow::Owned(input.to_uppercase()),
    TextTransform::Lowercase => Cow::Owned(input.to_lowercase()),
    TextTransform::Capitalize => {
      let mut result = String::with_capacity(input.len());
      let mut start_of_word = true;
      for ch in input.chars() {
        if ch.is_alphabetic() {
          if start_of_word {
            result.extend(ch.to_uppercase());
            start_of_word = false;
          } else {
            result.extend(ch.to_lowercase());
          }
        } else {
          start_of_word = !ch.is_numeric();
          result.push(ch);
        }
      }
      Cow::Owned(result)
    }
  }
}

/// Construct a new string with an ellipsis appended such that it fits within `max_width`.
fn make_ellipsis_text<'s>(
  render_text: &'s str,
  start_index: usize,
  end_index: usize,
  font_style: &ResolvedFontStyle,
  global: &GlobalContext,
  max_width: f32,
) -> Cow<'s, str> {
  let mut truncated_text = &render_text[start_index..end_index];

  while !truncated_text.is_empty() {
    // try to calculate the last line only with the truncated text and ellipsis character
    let mut text_with_ellipsis = String::with_capacity(truncated_text.len() + ELLIPSIS_CHAR.len());

    text_with_ellipsis.push_str(truncated_text);
    text_with_ellipsis.push_str(ELLIPSIS_CHAR);

    let buffer = create_text_layout(&text_with_ellipsis, font_style, global, max_width, None);

    // if the text fits, return the text with ellipsis character
    if buffer.lines().count() == 1 {
      let before_last_line = &render_text[..start_index];

      // build the text with ellipsis character
      let mut text_with_ellipsis =
        String::with_capacity(before_last_line.len() + truncated_text.len() + ELLIPSIS_CHAR.len());

      text_with_ellipsis.push_str(before_last_line);
      text_with_ellipsis.push_str(truncated_text);
      text_with_ellipsis.push_str(ELLIPSIS_CHAR);

      return Cow::Owned(text_with_ellipsis);
    }

    // try to shrink by one char
    if let Some((char_idx, _)) = truncated_text.char_indices().last() {
      truncated_text = &truncated_text[..char_idx];
    } else {
      // the text is empty, break out
      break;
    }
  }

  // if there's nothing left, returns nothing
  Cow::Borrowed("")
}
