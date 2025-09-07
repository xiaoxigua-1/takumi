use std::{
  borrow::Cow,
  sync::{Arc, Mutex},
};

use parley::{
  Layout, LayoutContext, RangedBuilder, Run,
  fontique::{Blob, FallbackKey, FontInfoOverride, Script},
};
use swash::{
  FontRef,
  scale::{ScaleContext, Scaler},
};

#[cfg(feature = "woff")]
mod woff;

/// Errors that can occur during font loading and conversion.
#[derive(Debug)]
pub enum FontError {
  /// I/O error occurred while reading the font file
  Io(std::io::Error),
  /// Error occurred during WOFF2 to TTF conversion
  #[cfg(feature = "woff2")]
  Woff2(woff2_patched::decode::DecodeError),
  /// Error during woff parsing
  #[cfg(feature = "woff")]
  Woff(woff::WoffError),
  /// Unsupported Font Format
  UnsupportedFormat,
}

/// Supported font formats for loading and processing
#[derive(Copy, Clone)]
pub enum FontFormat {
  #[cfg(feature = "woff")]
  /// Web Open Font Format (WOFF) - compressed web font format
  Woff,
  #[cfg(feature = "woff2")]
  /// Web Open Font Format 2 (WOFF2) - improved compression web font format
  Woff2,
  /// TrueType Font format - standard desktop font format
  Ttf,
  /// OpenType Font format - extended font format with advanced typography
  Otf,
}

/// Loads and processes font data from raw bytes, optionally using format hint for detection
pub fn load_font<'source>(
  source: &'source [u8],
  format_hint: Option<FontFormat>,
) -> Result<Cow<'source, [u8]>, FontError> {
  let format = if let Some(format) = format_hint {
    format
  } else {
    guess_font_format(source)?
  };

  match format {
    FontFormat::Ttf | FontFormat::Otf => Ok(Cow::Borrowed(source)),
    #[cfg(feature = "woff2")]
    FontFormat::Woff2 => {
      let mut bytes = bytes::Bytes::copy_from_slice(source);

      let ttf = woff2_patched::convert_woff2_to_ttf(&mut bytes).map_err(FontError::Woff2)?;

      Ok(Cow::Owned(ttf))
    }
    #[cfg(feature = "woff")]
    FontFormat::Woff => Ok(Cow::Owned(
      woff::decompress_woff(source).map_err(FontError::Woff)?,
    )),
  }
}

fn guess_font_format(source: &[u8]) -> Result<FontFormat, FontError> {
  if source.len() < 4 {
    return Err(FontError::UnsupportedFormat);
  }

  match &source[0..4] {
    #[cfg(feature = "woff2")]
    b"wOF2" => Ok(FontFormat::Woff2),
    #[cfg(feature = "woff")]
    b"wOFF" => Ok(FontFormat::Woff),
    [0x00, 0x01, 0x00, 0x00] => Ok(FontFormat::Ttf),
    b"OTTO" => Ok(FontFormat::Otf),
    _ => Err(FontError::UnsupportedFormat),
  }
}

/// Embedded fonts
#[cfg(feature = "embed_fonts")]
const EMBEDDED_FONTS: &[&[u8]] = &[
  include_bytes!("../../../../assets/fonts/geist/GeistMono[wght].woff2"),
  include_bytes!("../../../../assets/fonts/geist/Geist[wght].woff2"),
];

/// A context for managing fonts in the rendering system.
pub struct FontContext {
  layout: Mutex<(parley::FontContext, LayoutContext<()>)>,
  scale: Mutex<ScaleContext>,
}

impl Default for FontContext {
  #[cfg(feature = "embed_fonts")]
  fn default() -> Self {
    Self::new_with_default_fonts()
  }

  #[cfg(not(feature = "embed_fonts"))]
  fn default() -> Self {
    Self::new()
  }
}

impl FontContext {
  /// Create a swash scaler and run the function with it
  /// The inner lock will be released after the function is executed
  pub fn with_scaler(&self, run: &Run<'_, ()>, func: impl FnOnce(&mut Scaler<'_>)) {
    let font = run.font();
    let font_ref = FontRef::from_index(font.data.as_ref(), font.index as usize).unwrap();

    let mut context = self.scale.lock().unwrap();

    let mut scaler = context
      .builder(font_ref)
      .size(run.font_size())
      .hint(true)
      .variations(run.synthesis().variations().iter().copied())
      .build();

    func(&mut scaler);
  }

  /// Access the inner context, then return the result of the function
  /// The inner lock will be released after the function is executed
  pub fn create_layout(
    &self,
    text: &str,
    func: impl FnOnce(&mut RangedBuilder<'_, ()>),
  ) -> Layout<()> {
    let mut lock = self.layout.lock().unwrap();
    let (fcx, lcx) = &mut *lock;

    let mut builder = lcx.ranged_builder(fcx, text, 1.0, true);

    func(&mut builder);

    builder.build(text)
  }

  /// Purge the rasterization cache.
  pub fn purge_cache(&self) {
    let mut lock = self.layout.lock().unwrap();
    lock.0.source_cache.prune(0, true);
  }

  /// Creates a new font context with option to opt-in load default fonts,
  /// only available when `embed_fonts` feature is enabled
  #[cfg(feature = "embed_fonts")]
  pub fn new_with_default_fonts() -> Self {
    let inner = Self::new();

    for font in EMBEDDED_FONTS {
      inner.load_and_store(font, None).unwrap();
    }

    inner
  }

  /// Creates a new font context.
  pub fn new() -> Self {
    Self {
      layout: Mutex::new((parley::FontContext::default(), LayoutContext::default())),
      scale: Mutex::new(ScaleContext::default()),
    }
  }

  /// Loads font into internal font db
  pub fn load_and_store(
    &self,
    source: &[u8],
    info_override: Option<FontInfoOverride<'_>>,
  ) -> Result<(), FontError> {
    let font_data = Blob::new(Arc::new(match load_font(source, None)? {
      Cow::Owned(vec) => vec,
      Cow::Borrowed(slice) => slice.to_vec(),
    }));

    let mut lock = self.layout.lock().unwrap();

    let fonts = lock.0.collection.register_fonts(font_data, info_override);

    for (family, _) in fonts {
      for (script, _) in Script::all_samples() {
        lock
          .0
          .collection
          .append_fallbacks(FallbackKey::new(*script, None), std::iter::once(family));
      }
    }

    Ok(())
  }
}
