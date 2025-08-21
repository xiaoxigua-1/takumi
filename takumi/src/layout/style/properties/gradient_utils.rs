use super::{Color, GradientStop, ResolvedGradientStop};
use std::collections::HashMap;

/// Resolves gradient steps into color stops with positions
pub(crate) fn resolve_gradient_stops(steps: &[GradientStop]) -> Vec<ResolvedGradientStop> {
  let mut resolved_stops = Vec::new();
  let mut last_position: Option<f32> = None;

  for step in steps {
    match step {
      GradientStop::ColorHint { color, hint: stop } => {
        let position = stop.or(last_position).unwrap_or(0.0);

        resolved_stops.push(ResolvedGradientStop {
          color: *color,
          position,
        });

        last_position = Some(position);
      }
      GradientStop::Hint(_hint_value) => {
        // Hints influence distribution but aren't color stops
      }
    }
  }

  // Handle stops without explicit positions
  if resolved_stops.len() > 1 {
    // Set last stop to 1.0 if it wasn't explicitly set
    if resolved_stops.last().map(|s| s.position) != Some(1.0)
      && steps
        .last()
        .map(|s| matches!(s, GradientStop::ColorHint { hint, .. } if hint.is_none()))
        .unwrap_or(false)
    {
      if let Some(last) = resolved_stops.last_mut() {
        last.position = 1.0;
      }
    }

    // Distribute evenly any stops without positions
    let mut i = 0;
    while i < resolved_stops.len() {
      if resolved_stops[i].position < 0.0
        || (i > 0 && resolved_stops[i].position <= resolved_stops[i - 1].position)
      {
        // Find next defined position
        let mut next_defined_index = i + 1;
        while next_defined_index < resolved_stops.len()
          && (resolved_stops[next_defined_index].position < 0.0
            || resolved_stops[next_defined_index].position
              <= resolved_stops[next_defined_index - 1].position)
        {
          next_defined_index += 1;
        }

        if next_defined_index < resolved_stops.len() {
          let start_pos = resolved_stops[i - 1].position;
          let end_pos = resolved_stops[next_defined_index].position;
          let segments = next_defined_index - i + 1;

          for j in 0..(next_defined_index - i) {
            let t = (j + 1) as f32 / segments as f32;
            resolved_stops[i + j].position = start_pos + t * (end_pos - start_pos);
          }
        } else {
          // No more defined positions, distribute to 1.0
          let start_pos = resolved_stops[i - 1].position;
          let segments = resolved_stops.len() - i + 1;

          for j in 0..(resolved_stops.len() - i) {
            let t = (j + 1) as f32 / segments as f32;
            resolved_stops[i + j].position = start_pos + t * (1.0 - start_pos);
          }
          break;
        }

        i = next_defined_index;
      } else {
        i += 1;
      }
    }
  } else if resolved_stops.len() == 1 && resolved_stops[0].position < 0.0 {
    // Only one stop, set to 0.0
    resolved_stops[0].position = 0.0;
  }

  // Ensure first stop is at 0.0 if not already
  if !resolved_stops.is_empty() && resolved_stops[0].position != 0.0 {
    resolved_stops[0].position = 0.0;
  }

  resolved_stops
}

/// Interpolates between two colors in RGBA space.
pub(crate) fn interpolate_rgba(c1: Color, c2: Color, t: f32) -> Color {
  let mut out = [0u8; 4];
  for ((&a_u8, &b_u8), out_i) in c1.0.iter().zip(c2.0.iter()).zip(out.iter_mut()) {
    let a = a_u8 as f32;
    let b = b_u8 as f32;
    *out_i = (a * (1.0 - t) + b * t).round() as u8;
  }
  Color(out)
}

/// Returns the color for a normalized position [0, 1] along the resolved stops.
/// Handles pixel-edge snapping, quantization caching, and bracketed interpolation.
pub(crate) fn color_from_stops(
  mut position: f32,
  pixel_epsilon: f32,
  resolved_stops: &[ResolvedGradientStop],
  cache: &mut HashMap<u32, Color>,
) -> Color {
  if resolved_stops.is_empty() {
    return Color([0, 0, 0, 0]);
  }
  if resolved_stops.len() == 1 {
    return resolved_stops[0].color;
  }

  position = position.clamp(0.0, 1.0);

  // Snap to exact ends when within one pixel of the edges to match expectations
  if position <= pixel_epsilon {
    position = 0.0;
  } else if (1.0 - position) <= pixel_epsilon {
    position = 1.0;
  }

  // Quantize position for caching
  let quantized_key = (position * 65535.0).round() as u32;
  if let Some(color) = cache.get(&quantized_key) {
    return *color;
  }

  // Find the two stops that bracket the current position
  let mut left_index = 0usize;
  for (i, stop) in resolved_stops.iter().enumerate() {
    if stop.position <= position {
      left_index = i;
    } else {
      break;
    }
  }

  let color = if left_index >= resolved_stops.len() - 1 {
    resolved_stops[resolved_stops.len() - 1].color
  } else {
    let left_stop = &resolved_stops[left_index];
    let right_stop = &resolved_stops[left_index + 1];
    let segment_length = (right_stop.position - left_stop.position).max(1e-6);
    let local_t = ((position - left_stop.position) / segment_length).clamp(0.0, 1.0);
    interpolate_rgba(left_stop.color, right_stop.color, local_t)
  };

  cache.insert(quantized_key, color);
  color
}
