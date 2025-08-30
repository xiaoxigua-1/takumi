use std::borrow::Cow;

use cosmic_text::{Attrs, Buffer, Metrics, Shaping};
use taffy::{Layout, Point, Size};

use crate::{
  GlobalContext,
  layout::style::{Color, FontStyle, TextOverflow, TextTransform},
  rendering::{Canvas, RenderContext},
};

const ELLIPSIS_CHAR: &str = "…";

/// Draws text on the canvas with the specified font style and layout.
pub fn draw_text(
  text: &str,
  style: &FontStyle,
  context: &RenderContext,
  canvas: &Canvas,
  layout: Layout,
) {
  if style.font_size == 0.0 {
    return;
  }

  let content_box = layout.content_box_size();

  let start_x = layout.content_box_x();
  let start_y = layout.content_box_y();

  let render_text = apply_text_transform(text, style.text_transform);

  let buffer = construct_text_buffer(
    &render_text,
    style,
    context.global,
    Some((Some(content_box.width), Some(content_box.height))),
  );

  let Some(last_run) = buffer.layout_runs().last() else {
    // No runs, nothing to draw
    return;
  };

  let Some(last_glyph) = last_run.glyphs.last() else {
    // No runs, nothing to draw
    return;
  };

  let should_append_ellipsis =
    style.text_overflow == TextOverflow::Ellipsis && last_glyph.end < render_text.len();

  if should_append_ellipsis {
    let first_glyph = last_run.glyphs.first().unwrap();

    let mut truncated_text = &render_text[first_glyph.start..last_glyph.end];

    while !truncated_text.is_empty() {
      let mut text_with_ellipsis =
        String::with_capacity(truncated_text.len() + ELLIPSIS_CHAR.len());

      text_with_ellipsis.push_str(truncated_text);
      text_with_ellipsis.push_str(ELLIPSIS_CHAR);

      let truncated_buffer =
        construct_text_buffer(&text_with_ellipsis, style, context.global, None);

      let last_line = truncated_buffer.layout_runs().last().unwrap();

      if last_line.line_w <= content_box.width {
        break;
      }

      truncated_text = &truncated_text[..truncated_text.len() - ELLIPSIS_CHAR.len()];
    }

    let before_last_line = &render_text[..first_glyph.start];

    let mut text_with_ellipsis =
      String::with_capacity(before_last_line.len() + truncated_text.len() + ELLIPSIS_CHAR.len());

    text_with_ellipsis.push_str(before_last_line);
    text_with_ellipsis.push_str(truncated_text);
    text_with_ellipsis.push_str(ELLIPSIS_CHAR);

    return draw_text(&text_with_ellipsis, style, context, canvas, layout);
  }

  let transform_origin = Point {
    x: (layout.location.x + layout.size.width / 2.0) as i32,
    y: (layout.location.y + layout.size.height / 2.0) as i32,
  };

  draw_buffer(
    context,
    &buffer,
    canvas,
    style.color,
    (start_x, start_y),
    transform_origin,
  );
}

fn draw_buffer(
  context: &RenderContext,
  buffer: &Buffer,
  canvas: &Canvas,
  color: Color,
  (start_x, start_y): (f32, f32),
  transform_origin: Point<i32>,
) {
  let mut font_system = context.global.font_context.font_system.lock().unwrap();
  let mut font_cache = context.global.font_context.font_cache.lock().unwrap();

  for run in buffer.layout_runs() {
    for glyph in run.glyphs.iter() {
      let physical_glyph = glyph.physical((0., 0.), 1.0);

      let Some(image) = font_cache.get_image(&mut font_system, physical_glyph.cache_key) else {
        continue; // No image for this glyph, skip
      };

      let base_offset = Point {
        x: physical_glyph.x + image.placement.left + start_x as i32,
        y: run.line_y as i32 + physical_glyph.y - image.placement.top + start_y as i32,
      };

      let mut image_data = image.data.clone();

      match image.content {
        cosmic_text::SwashContent::Mask => {
          canvas.draw_mask(
            image_data,
            base_offset,
            Size {
              width: image.placement.width,
              height: image.placement.height,
            },
            color,
            transform_origin,
            *context.rotation,
          );
        }
        cosmic_text::SwashContent::Color => {
          // apply alpha to the image based on the glyph color alpha
          if color.0[3] != 255 {
            let target_alpha = color.0[3] as f32 / 255.0;

            for alpha in image_data.iter_mut().skip(3).step_by(4) {
              *alpha = (*alpha as f32 * target_alpha) as u8;
            }
          }

          canvas.draw_mask(
            image_data,
            base_offset,
            Size {
              width: image.placement.width,
              height: image.placement.height,
            },
            color,
            transform_origin,
            *context.rotation,
          );
        }
        _ => {}
      }
    }
  }
}

pub(crate) fn construct_text_buffer(
  text: &str,
  font_style: &FontStyle,
  global: &GlobalContext,
  size: Option<(Option<f32>, Option<f32>)>,
) -> Buffer {
  let metrics = Metrics::new(font_style.font_size, font_style.line_height);
  let mut buffer = Buffer::new_empty(metrics);

  let mut attrs = Attrs::new().weight(font_style.font_weight);

  attrs = attrs.style(font_style.text_style.into());

  if let Some(font_family) = font_style.font_family.as_ref() {
    attrs = attrs.family(font_family.as_family());
  }

  if let Some(letter_spacing) = font_style.letter_spacing {
    attrs = attrs.letter_spacing(letter_spacing);
  }

  let mut font_system = global.font_context.font_system.lock().unwrap();

  if let Some((width, height)) = size {
    buffer.set_size(&mut font_system, width, height);
  }

  let text = apply_text_transform(text, font_style.text_transform);

  buffer.set_rich_text(
    &mut font_system,
    [(text.as_ref(), attrs.clone())],
    &attrs,
    Shaping::Advanced,
    font_style.text_align,
  );

  buffer
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
