use std::{
  borrow::Cow,
  sync::{Arc, Mutex, atomic::AtomicUsize},
};

use cosmic_text::{
  FontSystem, SwashCache,
  fontdb::{Database, Source},
};
use parley::fontique::Blob;

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

/// A context for managing fonts in the rendering system.
///
/// This struct holds the font system and cache used for text rendering.
pub struct FontContext {
  inner: Mutex<parley::FontContext>,
  counter: AtomicUsize,
}

/// Embedded fonts
#[cfg(feature = "embed_fonts")]
const EMBEDDED_FONTS: &[&[u8]] = &[include_bytes!(
  "../../../../assets/fonts/plus-jakarta-sans/PlusJakartaSans-VariableFont_wght.woff2"
)];

impl Default for FontContext {
  fn default() -> Self {
    Self(Mutex::new(parley::FontContext::new()))
  }
}

impl FontContext {
  /// Purge the rasterization cache.
  pub fn purge_cache(&self) {
    let mut lock = self.0.lock().unwrap();
    lock.source_cache.prune(0, true);
  }

  /// Creates a new font context with option to opt-in load default fonts,
  /// only available when `embed_fonts` feature is enabled
  #[cfg(feature = "embed_fonts")]
  pub fn new(load_default_fonts: bool) -> Self {
    let context = Self::default();

    if load_default_fonts {
      for font in EMBEDDED_FONTS {
        context.load_and_store(font).unwrap();
      }
    }

    context
  }

  /// Creates a new font context.
  #[cfg(not(feature = "embed_fonts"))]
  pub fn new() -> Self {
    Self::default()
  }

  /// Loads font into internal font db
  pub fn load_and_store(&self, source: &[u8]) -> Result<(), FontError> {
    let font_data = load_font(source, None)?;

    let mut lock = self.0.lock().unwrap();

    let db_mut = lock.collection.register_fonts(font_data, None)?;

    // Wrap the font bytes in a single Arc so the database can parse faces
    // (including font collections) without per-face copying.
    let arc_data = Blob::from_raw_parts(
      match font_data {
        Cow::Owned(vec) => vec,
        Cow::Borrowed(slice) => slice.to_vec(),
      },
      self.counter.fetch_add(1, Ordering::Relaxed),
    );

    db_mut.load_font_source(Source::Binary(arc_data));

    Ok(())
  }
}
