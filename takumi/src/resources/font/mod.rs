use std::sync::Mutex;

use cosmic_text::{FontSystem, SwashCache, fontdb::Database};

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
pub fn load_font(source: Vec<u8>, format_hint: Option<FontFormat>) -> Result<Vec<u8>, FontError> {
  let format = if let Some(format) = format_hint {
    format
  } else {
    guess_font_format(&source)?
  };

  match format {
    FontFormat::Ttf | FontFormat::Otf => Ok(source),
    #[cfg(feature = "woff2")]
    FontFormat::Woff2 => {
      let mut bytes = bytes::Bytes::from(source);

      woff2_patched::convert_woff2_to_ttf(&mut bytes).map_err(FontError::Woff2)
    }
    #[cfg(feature = "woff")]
    FontFormat::Woff => woff::decompress_woff(&source).map_err(FontError::Woff),
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
#[derive(Debug)]
pub struct FontContext {
  /// The font system used for text layout and rendering
  pub font_system: Mutex<FontSystem>,
  /// The cache for font glyphs and metrics
  pub font_cache: Mutex<SwashCache>,
}

impl Default for FontContext {
  fn default() -> Self {
    Self {
      font_system: Mutex::new(FontSystem::new_with_locale_and_db(
        "en-US".to_string(),
        Database::new(),
      )),
      font_cache: Mutex::new(SwashCache::new()),
    }
  }
}

impl FontContext {
  /// Loads font into internal font db
  pub fn load_and_store(&self, source: Vec<u8>) -> Result<(), FontError> {
    let font_data = load_font(source, None)?;

    let mut lock = self.font_system.lock().unwrap();
    lock.db_mut().load_font_data(font_data);

    Ok(())
  }
}
