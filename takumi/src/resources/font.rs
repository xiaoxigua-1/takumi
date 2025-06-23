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
  Woff,
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
    FontFormat::Woff => decompress_woff(source),
  }
}

fn guess_font_format(source: &[u8]) -> Result<FontFormat, FontError> {
  if source.len() < 4 {
    return Err(FontError::UnsupportedFormat);
  }

  match &source[0..4] {
    b"wOF2" => Ok(FontFormat::Woff2),
    b"wOFF" => Ok(FontFormat::Woff),
    [0x00, 0x01, 0x00, 0x00] => Ok(FontFormat::Ttf),
    b"OTTO" => Ok(FontFormat::Otf),
    _ => Err(FontError::UnsupportedFormat),
  }
}

#[cfg(feature = "woff")]
fn decompress_woff(data: Vec<u8>) -> Result<Vec<u8>, FontError> {
  if data.len() < 44 {
    return Err(FontError::UnsupportedFormat);
  }

  // WOFF header parsing (simplified)
  let compressed_size = u32::from_be_bytes([data[8], data[9], data[10], data[11]]) as usize;

  let decompressed_size = u32::from_be_bytes([data[12], data[13], data[14], data[15]]) as usize;

  // Skip WOFF header (44 bytes) and decompress
  let compressed_data = &data[44..44 + compressed_size];

  // Use flate2 for zlib decompression
  use flate2::read::ZlibDecoder;
  use std::io::Read;

  let mut decoder = ZlibDecoder::new(compressed_data);
  let mut decompressed = Vec::with_capacity(decompressed_size);
  decoder
    .read_to_end(&mut decompressed)
    .map_err(FontError::Io)?;

  Ok(decompressed)
}
