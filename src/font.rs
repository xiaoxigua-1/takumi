use std::sync::LazyLock;

use ab_glyph::FontVec;
use bytes::Bytes;

pub static FONT: LazyLock<FontVec> = LazyLock::new(|| {
  let woff_data = include_bytes!("../assets/noto-sans-tc-v36-chinese-traditional_latin-700.woff2");
  let mut font = Bytes::from_static(woff_data);

  let ttf = woff2_patched::convert_woff2_to_ttf(&mut font).unwrap();
  FontVec::try_from_vec(ttf).unwrap()
});
