use std::{fs::read, path::Path};

use bytes::Bytes;
use woff2_patched::convert_woff2_to_ttf;

#[derive(Debug)]
pub enum FontError {
  Io(std::io::Error),
  Woff2(woff2_patched::decode::DecodeError),
}

pub fn load_woff2_font(font_file: &Path) -> Result<Vec<u8>, FontError> {
  let woff_data = read(font_file).map_err(FontError::Io)?;
  let mut font = Bytes::from(woff_data);

  convert_woff2_to_ttf(&mut font).map_err(FontError::Woff2)
}
