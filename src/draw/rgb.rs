use image::Rgba;

pub fn parse_rgb(rgb: u32) -> Rgba<u8> {
  let red = ((rgb >> 16) & 0xFF) as u8;
  let green = ((rgb >> 8) & 0xFF) as u8;
  let blue = (rgb & 0xFF) as u8;

  Rgba([red, green, blue, 255])
}
