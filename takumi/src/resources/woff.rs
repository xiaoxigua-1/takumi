use flate2::bufread::ZlibDecoder;
use std::io::Read;

use crate::FontError;

const WOFF_MAGIC: u32 = 0x774F4646; // 'wOFF'

#[derive(Debug)]
pub enum WoffError {
  InvalidMagic,
  InvalidData,
  CompressionError,
  TooShort,
}

impl From<WoffError> for FontError {
  fn from(value: WoffError) -> Self {
    FontError::Woff(value)
  }
}

pub fn decompress_woff(woff_data: &[u8]) -> Result<Vec<u8>, WoffError> {
  if woff_data.len() < 44 {
    return Err(WoffError::TooShort);
  }

  // Parse WOFF header
  let magic = u32::from_be_bytes([woff_data[0], woff_data[1], woff_data[2], woff_data[3]]);
  if magic != WOFF_MAGIC {
    return Err(WoffError::InvalidMagic);
  }

  let flavor = u32::from_be_bytes([woff_data[4], woff_data[5], woff_data[6], woff_data[7]]);
  let num_tables = u16::from_be_bytes([woff_data[12], woff_data[13]]);
  let total_sfnt_size =
    u32::from_be_bytes([woff_data[16], woff_data[17], woff_data[18], woff_data[19]]);

  // Create TTF output buffer
  let mut ttf_data = vec![0u8; total_sfnt_size as usize];

  // Write TTF header (12 bytes)
  ttf_data[0..4].copy_from_slice(&flavor.to_be_bytes());
  ttf_data[4..6].copy_from_slice(&num_tables.to_be_bytes());

  // Calculate search range, entry selector, and range shift
  let mut search_range = 1u16;
  let mut entry_selector = 0u16;
  while search_range <= num_tables / 2 {
    search_range <<= 1;
    entry_selector += 1;
  }
  search_range <<= 4;
  let range_shift = num_tables * 16 - search_range;

  ttf_data[6..8].copy_from_slice(&search_range.to_be_bytes());
  ttf_data[8..10].copy_from_slice(&entry_selector.to_be_bytes());
  ttf_data[10..12].copy_from_slice(&range_shift.to_be_bytes());

  // Parse table directory entries and write TTF table directory
  let mut tables = Vec::new();
  let mut ttf_offset = 12 + (num_tables as usize * 16); // After header + table directory

  for i in 0..num_tables as usize {
    let entry_start = 44 + i * 20; // WOFF header is 44 bytes, each entry is 20 bytes

    let tag = u32::from_be_bytes([
      woff_data[entry_start],
      woff_data[entry_start + 1],
      woff_data[entry_start + 2],
      woff_data[entry_start + 3],
    ]);
    let offset = u32::from_be_bytes([
      woff_data[entry_start + 4],
      woff_data[entry_start + 5],
      woff_data[entry_start + 6],
      woff_data[entry_start + 7],
    ]);
    let comp_length = u32::from_be_bytes([
      woff_data[entry_start + 8],
      woff_data[entry_start + 9],
      woff_data[entry_start + 10],
      woff_data[entry_start + 11],
    ]);
    let orig_length = u32::from_be_bytes([
      woff_data[entry_start + 12],
      woff_data[entry_start + 13],
      woff_data[entry_start + 14],
      woff_data[entry_start + 15],
    ]);
    let checksum = u32::from_be_bytes([
      woff_data[entry_start + 16],
      woff_data[entry_start + 17],
      woff_data[entry_start + 18],
      woff_data[entry_start + 19],
    ]);

    // Write TTF table directory entry
    let ttf_dir_start = 12 + i * 16;
    ttf_data[ttf_dir_start..ttf_dir_start + 4].copy_from_slice(&tag.to_be_bytes());
    ttf_data[ttf_dir_start + 4..ttf_dir_start + 8].copy_from_slice(&checksum.to_be_bytes());
    ttf_data[ttf_dir_start + 8..ttf_dir_start + 12]
      .copy_from_slice(&(ttf_offset as u32).to_be_bytes());
    ttf_data[ttf_dir_start + 12..ttf_dir_start + 16].copy_from_slice(&orig_length.to_be_bytes());

    tables.push((tag, offset, comp_length, orig_length, ttf_offset));
    ttf_offset += ((orig_length + 3) & !3) as usize; // 4-byte aligned
  }

  // Sort tables by tag (required in TTF format)
  tables.sort_by_key(|&(tag, _, _, _, _)| tag);

  // Update table directory with sorted offsets
  let mut current_offset = 12 + (num_tables as usize * 16);
  for (i, &(_tag, _, _, orig_length, _)) in tables.iter().enumerate() {
    let ttf_dir_start = 12 + i * 16;
    ttf_data[ttf_dir_start + 8..ttf_dir_start + 12]
      .copy_from_slice(&(current_offset as u32).to_be_bytes());
    current_offset += ((orig_length + 3) & !3) as usize;
  }

  // Extract and decompress table data
  let mut current_offset = 12 + (num_tables as usize * 16);
  for &(_, woff_offset, comp_length, orig_length, _) in &tables {
    let table_start = woff_offset as usize;
    let table_end = table_start + comp_length as usize;

    if table_end > woff_data.len() {
      return Err(WoffError::InvalidData);
    }

    let table_data = &woff_data[table_start..table_end];

    if comp_length != orig_length {
      // Compressed - decompress using zlib
      let mut decoder = ZlibDecoder::new(table_data);
      let mut decompressed = Vec::new();
      decoder
        .read_to_end(&mut decompressed)
        .map_err(|_| WoffError::CompressionError)?;

      if decompressed.len() != orig_length as usize {
        return Err(WoffError::CompressionError);
      }

      ttf_data[current_offset..current_offset + orig_length as usize]
        .copy_from_slice(&decompressed);
    } else {
      // Uncompressed - copy directly
      ttf_data[current_offset..current_offset + orig_length as usize].copy_from_slice(table_data);
    }

    current_offset += ((orig_length + 3) & !3) as usize; // 4-byte aligned
  }

  Ok(ttf_data)
}
