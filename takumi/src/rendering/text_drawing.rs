use std::borrow::Cow;

use cosmic_text::{Attrs, Buffer, Metrics, Shaping};
use image::{Pixel, Rgba};
use taffy::{Layout, Size};

use crate::{
  layout::style::{FontStyle, Gradient, LinearGradientOrColor, TextOverflow, TextTransform},
  rendering::{FastBlendImage, RenderContext},
};

const ELLIPSIS_CHAR: &str = "…";

/// Draws text on the canvas with the specified font style and layout.
pub fn draw_text(
  text: &str,
  style: &FontStyle,
  context: &RenderContext,
  canvas: &mut FastBlendImage,
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
    context,
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

      let truncated_buffer = construct_text_buffer(&text_with_ellipsis, style, context, None);

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

  draw_buffer(
    context,
    &buffer,
    canvas,
    content_box,
    &style.color,
    (start_x, start_y),
  );
}

fn draw_buffer(
  context: &RenderContext,
  buffer: &Buffer,
  canvas: &mut FastBlendImage,
  content_box: Size<f32>,
  color: &LinearGradientOrColor,
  (start_x, start_y): (f32, f32),
) {
  let mut font_system = context.global.font_context.font_system.lock().unwrap();
  let mut font_cache = context.global.font_context.font_cache.lock().unwrap();

  let mut gradient_ctx = if let LinearGradientOrColor::Gradient(gradient) = color {
    Some(gradient.to_draw_context(content_box.width, content_box.height, context))
  } else {
    None
  };

  for run in buffer.layout_runs() {
    for glyph in run.glyphs.iter() {
      let physical_glyph = glyph.physical((0., 0.), 1.0);

      let Some(image) = font_cache.get_image(&mut font_system, physical_glyph.cache_key) else {
        continue; // No image for this glyph, skip
      };

      let glyph_color =
        glyph
          .color_opt
          .map(|color| Rgba(color.as_rgba()))
          .or_else(|| match color {
            LinearGradientOrColor::Color(color) => Some((*color).into()),
            LinearGradientOrColor::Gradient(_) => None,
          });

      let base_x = physical_glyph.x + image.placement.left + start_x as i32;
      let base_y = run.line_y as i32 + physical_glyph.y - image.placement.top + start_y as i32;

      match image.content {
        cosmic_text::SwashContent::Mask => {
          let mut i = 0;
          for off_y in 0..image.placement.height as i32 {
            let final_y = base_y + off_y;

            if final_y < 0 || final_y >= canvas.height() as i32 {
              continue;
            }

            for off_x in 0..image.placement.width as i32 {
              let final_x = base_x + off_x;

              if final_x < 0 || final_x >= canvas.width() as i32 {
                continue;
              }

              let picked_color = if let Some(glyph_color) = glyph_color {
                glyph_color
              } else {
                match color {
                  LinearGradientOrColor::Gradient(gradient) => gradient
                    .at(
                      final_x as u32,
                      final_y as u32,
                      gradient_ctx.as_mut().unwrap(),
                    )
                    .into(),
                  LinearGradientOrColor::Color(_) => unreachable!(),
                }
              };

              let blended_color = match image.data[i] {
                255 => picked_color,
                alpha => {
                  let mut blended_color = Rgba(picked_color.0);

                  blended_color.0[3] = (blended_color.0[3] as f32 * (alpha as f32 / 255.0)) as u8;

                  blended_color
                }
              };

              canvas.draw_pixel(final_x as u32, final_y as u32, blended_color);

              i += 1;
            }
          }
        }
        cosmic_text::SwashContent::Color => {
          let mut i = 0;
          for off_y in 0..image.placement.height as i32 {
            let final_y = base_y + off_y;

            if final_y < 0 || final_y >= canvas.height() as i32 {
              continue;
            }

            for off_x in 0..image.placement.width as i32 {
              let final_x = base_x + off_x;

              if final_x < 0 || final_x >= canvas.width() as i32 {
                continue;
              }

              let picked_color = *Rgba::from_slice(image.data[i..i + 4].into());

              let blended_color = match glyph_color.map(|color| color.0[3]) {
                Some(255) | None => picked_color,
                Some(alpha) => {
                  let mut blended_color = Rgba(picked_color.0);

                  blended_color.0[3] *= (alpha as f32 / 255.0) as u8;

                  blended_color
                }
              };

              canvas.draw_pixel(final_x as u32, final_y as u32, blended_color);

              i += 4;
            }
          }
        }
        _ => {}
      }
    }
  }
}

pub(crate) fn construct_text_buffer(
  text: &str,
  font_style: &FontStyle,
  context: &RenderContext,
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

  let mut font_system = context.global.font_context.font_system.lock().unwrap();

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
