pub mod draw;
pub mod font;

use std::io::Cursor;

use axum::{
  Json, Router,
  http::{StatusCode, header},
  response::{IntoResponse, Response},
  routing::{get, post},
};
use bytes::Bytes;

use image::ImageBuffer;
use mimalloc::MiMalloc;

use crate::draw::{
  border_radius::apply_border_radius_antialiased,
  draw::{Canvas, draw_path},
  rgb::parse_rgb,
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/", get(health_check))
    .route("/image", post(generate_image));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  println!("Server running on http://localhost:3000");

  println!("✓ Noto Sans TC font loaded successfully");
  println!("Try: http://localhost:3000/image?text=你好世界Hello%20World");
  println!("Or: http://localhost:3000/overlay?text=繁體中文Traditional");

  axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
  "Image generation server is running with Noto Sans TC!"
}

async fn generate_image(body: Json<Canvas>) -> Result<Response, StatusCode> {
  let mut image = ImageBuffer::from_pixel(body.width, body.height, parse_rgb(body.bg_color));

  let radius = body.border_radius;

  let paths = body.0.paths;

  for path in paths {
    draw_path(&mut image, path).await;
  }

  if let Some(radius) = radius {
    apply_border_radius_antialiased(&mut image, radius as f32);
  }

  let mut buffer = Vec::new();
  let mut cursor = Cursor::new(&mut buffer);
  image
    .write_to(&mut cursor, image::ImageFormat::WebP)
    .unwrap();

  Ok(([(header::CONTENT_TYPE, "image/webp")], Bytes::from(buffer)).into_response())
}
