use cosmic_text::{Attrs, Buffer, Family, Metrics, Shaping};
use image::Rgba;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use taffy::Layout;

use crate::{
  core::RenderContext,
  rendering::FastBlendImage,
  style::{ColorAt, ResolvedFontStyle},
};

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
  let start_y = layout.content_box_y() + style.font_size * ((style.line_height - 1.0) / 2.0);

  let mut buffer = construct_text_buffer(text, style, context);

  let mut font_system = context.global.font_context.font_system.lock().unwrap();

  buffer.set_size(
    &mut font_system,
    Some(content_box.width),
    Some(content_box.height),
  );

  let mut font_cache = context.global.font_context.font_cache.lock().unwrap();

  buffer.draw(
    &mut font_system,
    &mut font_cache,
    cosmic_text::Color(0),
    |x, y, w, h, color| {
      let color = color.as_rgba();

      let text_alpha = color[3] as f32 / 255.0;

      if text_alpha == 0.0 {
        return;
      }

      // FIXME: emojis with rich coloring with black might not be rendered correctly.
      let mut render_color: Rgba<u8> = if color[0] == 0 && color[1] == 0 && color[2] == 0 {
        style
          .color
          .at(content_box.width, content_box.height, x as u32, y as u32)
          .into()
      } else {
        Rgba(color)
      };

      render_color.0[3] = (render_color.0[3] as f32 * text_alpha) as u8;

      draw_filled_rect_mut(
        canvas,
        Rect::at(start_x as i32 + x, start_y as i32 + y).of_size(w, h),
        render_color,
      );
    },
  );
}

pub(crate) fn construct_text_buffer(
  text: &str,
  font_style: &ResolvedFontStyle,
  context: &RenderContext,
) -> Buffer {
  let metrics = Metrics::relative(font_style.font_size, font_style.line_height);
  let mut buffer = Buffer::new_empty(metrics);

  let mut attrs = Attrs::new().weight(font_style.font_weight);

  if let Some(font_family) = font_style.font_family.as_ref() {
    attrs = attrs.family(Family::Name(font_family));
  }

  if let Some(letter_spacing) = font_style.letter_spacing {
    attrs = attrs.letter_spacing(letter_spacing);
  }

  let mut font_system = context.global.font_context.font_system.lock().unwrap();

  buffer.set_rich_text(
    &mut font_system,
    [(text, attrs.clone())],
    &attrs,
    Shaping::Advanced,
    font_style.text_align,
  );

  buffer
}
