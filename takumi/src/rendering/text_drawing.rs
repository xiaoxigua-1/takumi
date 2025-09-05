use std::borrow::Cow;
use std::sync::Arc;

use image::RgbaImage;
use parley::{PositionedLayoutItem, StyleProperty};
use swash::{
  Setting,
  scale::{Render, Source, StrikeWith, image::Content},
  tag_from_bytes,
};
use taffy::{Layout, Point};
use zeno::Format;

use crate::{
  GlobalContext,
  layout::style::{Affine, Color, ResolvedFontStyle, Style, TextOverflow, TextTransform},
  rendering::{BorderRadius, Canvas, RenderContext, overlay_image, resolve_layers_tiles},
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

  let start_x = layout.content_box_x();
  let start_y = layout.content_box_y();

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
          )
        }
      }
    }

    draw_buffer(
      context,
      &buffer,
      canvas,
      style.inheritable_style.color.unwrap_or_else(Color::black),
      (start_x, start_y),
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
    (start_x, start_y),
    layout,
    None,
  );
}

fn draw_buffer(
  context: &RenderContext,
  buffer: &parley::Layout<()>,
  canvas: &Canvas,
  color: Color,
  (start_x, start_y): (f32, f32),
  _layout: Layout,
  image_fill: Option<RgbaImage>,
) {
  for line in buffer.lines() {
    for item in line.items() {
      let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
        continue;
      };

      let run = glyph_run.run();

      context.global.font_context.with_scaler(run, |scaler| {
        for glyph in glyph_run.positioned_glyphs() {
          let Some(mut image) = Render::new(&[
            Source::ColorOutline(0),
            Source::ColorBitmap(StrikeWith::BestFit),
            Source::Outline,
          ])
          .format(Format::Alpha)
          .render(scaler, glyph.id) else {
            continue;
          };

          image.placement.left += (glyph.x + start_x) as i32;
          image.placement.top = (start_y + glyph.y) as i32 - image.placement.top;

          match (image.content, image_fill.clone()) {
            (Content::Mask, Some(fill)) => {
              // TODO: fix this
              canvas.draw_mask(image.data, image.placement, color, Some(fill));
            }
            (Content::Mask, None) => canvas.draw_mask(image.data, image.placement, color, None),
            (Content::Color, Some(fill)) => {
              canvas.draw_mask(
                // collect the alpha values from [r, g, b, a] sequence
                image.data.into_iter().skip(3).step_by(4).collect(),
                image.placement,
                color,
                Some(fill),
              );
            }
            (Content::Color, None) => {
              // apply alpha to the image based on the glyph color alpha
              if color.0[3] != 255 {
                let target_alpha = color.0[3] as f32 / 255.0;

                for alpha in image.data.iter_mut().skip(3).step_by(4) {
                  *alpha = (*alpha as f32 * target_alpha) as u8;
                }
              }

              let offset = Point {
                x: image.placement.left,
                y: image.placement.top,
              };

              canvas.overlay_image(
                Arc::new(
                  RgbaImage::from_raw(image.placement.width, image.placement.height, image.data)
                    .unwrap(),
                ),
                offset,
                BorderRadius::default(),
                context.transform,
              );
            }
            _ => {}
          }
        }
      });
    }
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
    builder.push_default(StyleProperty::FontVariations(parley::FontSettings::List(
      Cow::Borrowed(&[Setting {
        tag: VARIABLE_FONT_WEIGHT_TAG,
        value: font_style.font_weight.value(),
      }]),
    )));

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
