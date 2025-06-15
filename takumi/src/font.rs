use std::{fs::read, path::Path};

use bytes::Bytes;
use woff2_patched::convert_woff2_to_ttf;

/// Errors that can occur during font loading and conversion.
#[derive(Debug)]
pub enum FontError {
  /// I/O error occurred while reading the font file
  Io(std::io::Error),
  /// Error occurred during WOFF2 to TTF conversion
  Woff2(woff2_patched::decode::DecodeError),
}

/// Loads a WOFF2 font file and converts it to TTF format.
pub fn load_woff2_font(font_file: &Path) -> Result<Vec<u8>, FontError> {
  let woff_data = read(font_file).map_err(FontError::Io)?;
  let mut font = Bytes::from(woff_data);

  convert_woff2_to_ttf(&mut font).map_err(FontError::Woff2)
}
