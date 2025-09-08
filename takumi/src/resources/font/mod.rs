use std::{
  borrow::Cow,
  collections::{HashMap, HashSet},
  sync::{Arc, Mutex},
};

use parley::{
  GenericFamily, Layout, LayoutContext, RangedBuilder, Run,
  fontique::{Blob, FallbackKey, FontInfoOverride, Script},
};
use swash::{
  FontRef,
  scale::{ScaleContext, image::Image, outline::Outline},
};

#[cfg(feature = "woff")]
mod woff;

/// Represents a resolved glyph that can be either a bitmap image or an outline
#[derive(Clone)]
pub enum ResolvedGlyph {
  /// A bitmap glyph image
  Image(Image),
  /// A vector outline glyph
  Outline(Outline),
}

/// Thread-safe reference-counted glyph for caching
pub type CachedGlyph = Arc<ResolvedGlyph>;

/// Cache key for glyph resolution
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GlyphCacheKey {
  /// Font identifier
  pub font_id: u32,
  /// Glyph identifier
  pub glyph_id: u16,
  /// Font size (quantized to reduce cache fragmentation)
  pub font_size: u16,
  /// Hash of font variations
  pub variations_hash: u64,
}

/// Combined font scaling and caching context
pub struct FontScaleCache {
  /// Swash scale context
  scale: ScaleContext,
  /// LRU glyph cache for resolved glyphs
  glyph_cache: GlyphCache,
}

/// LRU glyph cache for resolved glyphs
pub struct GlyphCache {
  /// Maximum number of entries to keep in cache
  max_entries: usize,
  /// Cache entries with access order tracking
  entries: HashMap<GlyphCacheKey, (CachedGlyph, usize)>,
  /// Access counter for LRU eviction
  access_counter: usize,
}

impl Default for GlyphCache {
  fn default() -> Self {
    Self {
      max_entries: 10000, // Configurable cache size
      entries: HashMap::new(),
      access_counter: 0,
    }
  }
}

impl GlyphCache {
  /// Get a glyph from the cache, updating access order
  pub fn get(&mut self, key: &GlyphCacheKey) -> Option<CachedGlyph> {
    if let Some((glyph, access_count)) = self.entries.get_mut(key) {
      *access_count = self.access_counter;
      self.access_counter += 1;
      Some(Arc::clone(glyph))
    } else {
      None
    }
  }

  /// Insert a glyph into the cache with LRU eviction
  pub fn insert(&mut self, key: GlyphCacheKey, glyph: ResolvedGlyph) {
    // Evict least recently used entries if cache is full
    if self.entries.len() >= self.max_entries {
      let mut oldest_key = None;
      let mut oldest_access = usize::MAX;

      for (k, (_, access_count)) in &self.entries {
        if *access_count < oldest_access {
          oldest_access = *access_count;
          oldest_key = Some(*k);
        }
      }

      if let Some(oldest_key) = oldest_key {
        self.entries.remove(&oldest_key);
      }
    }

    self
      .entries
      .insert(key, (Arc::new(glyph), self.access_counter));
    self.access_counter += 1;
  }

  /// Clear all cached glyphs
  pub fn clear(&mut self) {
    self.entries.clear();
    self.access_counter = 0;
  }

  /// Get cache statistics
  pub fn stats(&self) -> (usize, usize) {
    (self.entries.len(), self.max_entries)
  }
}

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
const EMBEDDED_FONTS: &[(&[u8], &str, GenericFamily)] = &[
  (
    include_bytes!("../../../../assets/fonts/geist/Geist[wght].woff2"),
    "Geist",
    GenericFamily::SansSerif,
  ),
  (
    include_bytes!("../../../../assets/fonts/geist/GeistMono[wght].woff2"),
    "Geist Mono",
    GenericFamily::Monospace,
  ),
];

/// A context for managing fonts in the rendering system.
pub struct FontContext {
  layout: Mutex<(parley::FontContext, LayoutContext<()>)>,
  scale_cache: Mutex<FontScaleCache>,
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
  /// Generate a cache key for glyph resolution
  fn create_cache_key(&self, run: &Run<'_, ()>, glyph_id: u16) -> GlyphCacheKey {
    let font = run.font();
    let synthesis = run.synthesis();
    let variations = synthesis.variations();

    let mut variations_hash = 0u64;
    for variation in variations {
      variations_hash = variations_hash
        .wrapping_mul(31)
        .wrapping_add(variation.tag as u64);
      variations_hash = variations_hash
        .wrapping_mul(31)
        .wrapping_add(variation.value.to_bits() as u64);
    }

    GlyphCacheKey {
      font_id: font.index,
      glyph_id,
      font_size: (run.font_size() * 10.0) as u16, // Quantize to reduce cache fragmentation
      variations_hash,
    }
  }

  /// Get or resolve multiple glyphs using the cache
  /// Returns a HashMap of glyph_id -> CachedGlyph for efficient batch processing
  pub fn get_or_resolve_glyphs(
    &self,
    run: &Run<'_, ()>,
    glyph_ids: impl Iterator<Item = u16> + Clone,
  ) -> HashMap<u16, CachedGlyph> {
    // Collect unique glyph IDs to avoid duplicate work
    let unique_glyph_ids: HashSet<u16> = glyph_ids.collect();

    // Lock both scale and cache together for optimal performance
    let mut scale_cache = self.scale_cache.lock().unwrap();

    // Prepare font info for scaler creation
    let font = run.font();
    let font_ref = FontRef::from_index(font.data.as_ref(), font.index as usize).unwrap();

    let mut result = HashMap::new();

    // Process each unique glyph ID
    for &glyph_id in &unique_glyph_ids {
      let cache_key = self.create_cache_key(run, glyph_id);

      // Try to get from cache first
      if let Some(cached_glyph) = scale_cache.glyph_cache.get(&cache_key) {
        result.insert(glyph_id, cached_glyph);
        continue;
      }

      let mut scaler = scale_cache
        .scale
        .builder(font_ref)
        .size(run.font_size())
        .hint(true)
        .variations(run.synthesis().variations().iter().copied())
        .build();

      let resolved = scaler
        .scale_color_bitmap(glyph_id, swash::scale::StrikeWith::BestFit)
        .map(ResolvedGlyph::Image)
        .or_else(|| {
          scaler
            .scale_color_outline(glyph_id)
            .map(ResolvedGlyph::Outline)
        })
        .or_else(|| scaler.scale_outline(glyph_id).map(ResolvedGlyph::Outline));

      // Cache and return the result if we got one
      if let Some(glyph) = resolved {
        scale_cache.glyph_cache.insert(cache_key, glyph);
        // Get the cached version (now wrapped in Arc)
        if let Some(cached_glyph) = scale_cache.glyph_cache.get(&cache_key) {
          result.insert(glyph_id, cached_glyph);
        }
      }
    }

    result
  }

  /// Get or resolve a single glyph using the cache (backward compatibility)
  pub fn get_or_resolve_glyph(&self, run: &Run<'_, ()>, glyph_id: u16) -> Option<CachedGlyph> {
    self
      .get_or_resolve_glyphs(run, std::iter::once(glyph_id))
      .into_iter()
      .next()
      .map(|(_, glyph)| glyph)
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

  /// Clear the glyph cache
  pub fn purge_glyph_cache(&self) {
    let mut scale_cache = self.scale_cache.lock().unwrap();
    scale_cache.glyph_cache.clear();
  }

  /// Get glyph cache statistics (current_entries, max_entries)
  pub fn glyph_cache_stats(&self) -> (usize, usize) {
    let scale_cache = self.scale_cache.lock().unwrap();
    scale_cache.glyph_cache.stats()
  }

  /// Creates a new font context with option to opt-in load default fonts,
  /// only available when `embed_fonts` feature is enabled
  #[cfg(feature = "embed_fonts")]
  pub fn new_with_default_fonts() -> Self {
    let inner = Self::new();

    for (font, name, generic) in EMBEDDED_FONTS {
      inner
        .load_and_store(
          font,
          Some(FontInfoOverride {
            family_name: Some(name),
            ..Default::default()
          }),
          Some(*generic),
        )
        .unwrap();
    }

    inner
  }

  /// Creates a new font context.
  pub fn new() -> Self {
    Self {
      layout: Mutex::new((parley::FontContext::default(), LayoutContext::default())),
      scale_cache: Mutex::new(FontScaleCache {
        scale: ScaleContext::default(),
        glyph_cache: GlyphCache::default(),
      }),
    }
  }

  /// Loads font into internal font db
  pub fn load_and_store(
    &self,
    source: &[u8],
    info_override: Option<FontInfoOverride<'_>>,
    generic_family: Option<GenericFamily>,
  ) -> Result<(), FontError> {
    let font_data = Blob::new(Arc::new(match load_font(source, None)? {
      Cow::Owned(vec) => vec,
      Cow::Borrowed(slice) => slice.to_vec(),
    }));

    let mut lock = self.layout.lock().unwrap();

    let fonts = lock.0.collection.register_fonts(font_data, info_override);

    for (family, _) in fonts {
      if let Some(generic_family) = generic_family {
        lock
          .0
          .collection
          .append_generic_families(generic_family, std::iter::once(family));
      }

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
