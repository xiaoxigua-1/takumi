use std::sync::Mutex;

use cosmic_text::{FontSystem, SwashCache, fontdb::Database};

use crate::resources::{FontError, load_font};

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
  pub fn load_font(&self, source: Vec<u8>) -> Result<(), FontError> {
    let font_data = load_font(source, None)?;

    let mut lock = self.font_system.lock().unwrap();
    lock.db_mut().load_font_data(font_data);

    Ok(())
  }
}
