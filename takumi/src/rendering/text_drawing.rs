use cosmic_text::{Attrs, Buffer, Color, Metrics, Shaping};
use image::Rgba;
use imageproc::drawing::Canvas;
use taffy::{Layout, Size};

use crate::{
  ColorInput,
  core::RenderContext,
  rendering::FastBlendImage,
  style::{ColorAt, ResolvedFontStyle, TextOverflow},
};

const ELLIPSIS_CHAR: &str = "…";

/// Draws text on the canvas with the specified font style and layout.
pub fn draw_text(
  text: &str,
  style: &ResolvedFontStyle,
  context: &RenderContext,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  if style.color.is_transparent() || style.font_size == 0.0 {
    return;
  }

  let content_box = layout.content_box_size();

  let start_x = layout.content_box_x();
  let start_y = layout.content_box_y();

  let buffer = construct_text_buffer(
    text,
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
    style.text_overflow == TextOverflow::Ellipsis && last_glyph.end < text.len();

  if should_append_ellipsis {
    let first_glyph = last_run.glyphs.first().unwrap();

    let mut truncated_text = &text[first_glyph.start..last_glyph.end];

    while !truncated_text.is_empty() {
      let text_with_ellipsis = format!("{truncated_text}{ELLIPSIS_CHAR}");
      let truncated_buffer = construct_text_buffer(&text_with_ellipsis, style, context, None);

      let last_line = truncated_buffer.layout_runs().last().unwrap();

      if last_line.line_w <= content_box.width {
        break;
      }

      truncated_text = &truncated_text[..truncated_text.len() - ELLIPSIS_CHAR.len()];
    }

    let before_last_line = &text[..first_glyph.start];

    let text_with_ellipsis = format!("{before_last_line}{truncated_text}{ELLIPSIS_CHAR}");

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
  color: &ColorInput,
  (start_x, start_y): (f32, f32),
) {
  let mut font_system = context.global.font_context.font_system.lock().unwrap();
  let mut font_cache = context.global.font_context.font_cache.lock().unwrap();

  for run in buffer.layout_runs() {
    for glyph in run.glyphs.iter() {
      let physical_glyph = glyph.physical((0., 0.), 1.0);

      let glyph_color = match glyph.color_opt {
        Some(some) => some,
        None => Color(0),
      };

      font_cache.with_pixels(
        &mut font_system,
        physical_glyph.cache_key,
        glyph_color,
        |glyph_x, glyph_y, glyph_color| {
          if glyph_color.a() == 0 {
            return;
          }

          let x = physical_glyph.x + glyph_x;
          let y = run.line_y as i32 + physical_glyph.y + glyph_y;

          let glyph_color = glyph_color.as_rgba();

          let text_alpha = glyph_color[3] as f32 / 255.0;

          // FIXME: emojis with rich coloring with black might not be rendered correctly.
          let mut render_color: Rgba<u8> =
            if glyph_color[0] == 0 && glyph_color[1] == 0 && glyph_color[2] == 0 {
              color
                .at(content_box.width, content_box.height, x as u32, y as u32)
                .into()
            } else {
              Rgba(glyph_color)
            };

          render_color.0[3] = (render_color.0[3] as f32 * text_alpha) as u8;

          canvas.draw_pixel(
            start_x as u32 + x as u32,
            start_y as u32 + y as u32,
            render_color,
          );
        },
      );
    }
  }
}

pub(crate) fn construct_text_buffer(
  text: &str,
  font_style: &ResolvedFontStyle,
  context: &RenderContext,
  size: Option<(Option<f32>, Option<f32>)>,
) -> Buffer {
  let metrics = Metrics::new(font_style.font_size, font_style.line_height);
  let mut buffer = Buffer::new_empty(metrics);

  let mut attrs = Attrs::new().weight(font_style.font_weight);

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

  buffer.set_rich_text(
    &mut font_system,
    [(text, attrs.clone())],
    &attrs,
    Shaping::Advanced,
    font_style.text_align,
  );

  buffer
}
