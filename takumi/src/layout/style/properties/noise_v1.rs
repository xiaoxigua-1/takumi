use cssparser::Parser;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
  layout::style::{Color, FromCss, Gradient, ParseResult},
  rendering::RenderContext,
};

/// Procedural noise gradient that generates organic, natural-looking patterns using fractal Brownian motion.
/// This creates dynamic textures that can be used as backgrounds or overlays with customizable parameters
/// for controlling the noise characteristics and visual appearance.
#[derive(Debug, Clone, PartialEq, TS, Deserialize, Serialize)]
pub struct NoiseV1 {
  /// Controls the scale of the noise pattern. Higher values create finer, more detailed patterns
  pub frequency: Option<f64>,
  /// Random seed value that determines the unique noise pattern generated
  pub seed: Option<u32>,
  /// Number of noise layers combined to create complex patterns. More octaves add detail
  pub octaves: Option<usize>,
  /// Controls how much each octave contributes to the final pattern. Lower values create smoother patterns
  pub persistence: Option<f64>,
  /// Controls the frequency multiplier between octaves. Higher values create more varied patterns
  pub lacunarity: Option<f64>,
}

impl Gradient for NoiseV1 {
  type DrawContext = Fbm<Perlin>;

  fn at(&self, x: u32, y: u32, ctx: &Self::DrawContext) -> Color {
    let noise = ctx.get([x as f64, y as f64]);

    // Map noise from [-1, 1] to alpha range [0, 255] for subtle transparency variation
    let alpha = ((noise + 1.0) * 255.0).clamp(0.0, 255.0) as u8;

    Color([255, 255, 255, alpha])
  }

  fn to_draw_context(
    &self,
    _width: f32,
    _height: f32,
    _context: &RenderContext,
  ) -> Self::DrawContext {
    let mut fbm = Fbm::new(self.seed.unwrap_or(Fbm::<Perlin>::DEFAULT_SEED));

    if let Some(octaves) = self.octaves {
      fbm = fbm.set_octaves(octaves);
    }

    if let Some(persistence) = self.persistence {
      fbm = fbm.set_persistence(persistence);
    }

    if let Some(frequency) = self.frequency {
      fbm = fbm.set_frequency(frequency);
    }

    if let Some(lacunarity) = self.lacunarity {
      fbm = fbm.set_lacunarity(lacunarity);
    }

    fbm
  }
}

impl<'i> FromCss<'i> for NoiseV1 {
  /// noise-v1([<frequency>] [<octaves>] [<persistence>] [<lacunarity>] [<seed>])
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, NoiseV1> {
    let frequency = input
      .try_parse(Parser::expect_number)
      .map(|f| f as f64)
      .ok();
    let octaves = input
      .try_parse(Parser::expect_integer)
      .map(|i| i as usize)
      .ok();
    let persistence = input
      .try_parse(Parser::expect_number)
      .map(|f| f as f64)
      .ok();
    let lacunarity = input
      .try_parse(Parser::expect_number)
      .map(|f| f as f64)
      .ok();
    let seed = input
      .try_parse(Parser::expect_integer)
      .map(|i| i as u32)
      .ok();

    Ok(NoiseV1 {
      frequency,
      octaves,
      persistence,
      lacunarity,
      seed,
    })
  }
}
