use std::{collections::HashMap, fs::read, path::Path, sync::RwLock};

use ab_glyph::FontArc;
use bytes::Bytes;

#[derive(Debug)]
pub enum FontError {
  Io(std::io::Error),
  Woff2(woff2_patched::decode::DecodeError),
  Font(ab_glyph::InvalidFont),
}

pub fn load_woff2_font(font_file: &Path) -> Result<FontArc, FontError> {
  let woff_data = read(font_file).map_err(FontError::Io)?;
  let mut font = Bytes::from(woff_data);

  let ttf = woff2_patched::convert_woff2_to_ttf(&mut font).map_err(FontError::Woff2)?;
  FontArc::try_from_vec(ttf).map_err(FontError::Font)
}

#[derive(Default)]
pub struct FontStore {
  fonts: RwLock<HashMap<String, FontArc>>,
}

impl FontStore {
  pub fn load_woff2_font(&self, font_file: &Path, font_family: &str) -> Result<(), FontError> {
    let font = load_woff2_font(font_file)?;
    let mut fonts = self.fonts.write().unwrap();

    fonts.insert(font_family.to_string(), font);

    Ok(())
  }

  pub fn get_font(&self, font_family: &str) -> Option<FontArc> {
    let fonts = self.fonts.read().unwrap();
    fonts.get(font_family).cloned()
  }

  pub fn get_font_or_default(&self, font_family: &str) -> FontArc {
    self
      .get_font(font_family)
      .unwrap_or_else(|| self.default_font())
  }

  pub fn default_font(&self) -> FontArc {
    let fonts = self.fonts.read().unwrap();
    fonts
      .values()
      .next()
      .expect("No default font found")
      .clone()
  }
}
