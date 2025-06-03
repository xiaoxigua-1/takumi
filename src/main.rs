use axum::{
  Router,
  extract::{Json, State},
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::post,
};
use image::ImageFormat;
use imagen::{
  color::Color,
  context::Context,
  node::Node,
  render::{DrawProps, ImageRenderer, LayoutProps},
};
use serde::Deserialize;
use std::{io::Cursor, net::SocketAddr, path::Path, sync::Arc};
use tokio::net::TcpListener;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Deserialize)]
struct ImageRequest {
  pub width: u32,
  pub height: u32,
  pub background_color: Option<Color>,
  pub nodes: Vec<Node>,
}

async fn generate_image_handler(
  State(context): State<Arc<Context>>,
  Json(request): Json<ImageRequest>,
) -> Result<Response, StatusCode> {
  let renderer = ImageRenderer::new(
    DrawProps {
      background_color: request.background_color,
    },
    LayoutProps {
      width: request.width,
      height: request.height,
    },
  );

  let (mut taffy, node_ids) = renderer.create_taffy_tree(request.nodes);
  let image = renderer.draw(context.as_ref(), &mut taffy, node_ids);

  let mut buffer = Vec::new();
  let mut cursor = Cursor::new(&mut buffer);

  image.write_to(&mut cursor, ImageFormat::WebP).unwrap();

  Ok(([("content-type", "image/webp")], buffer).into_response())
}

#[tokio::main]
async fn main() {
  let context = Context::default();

  context
    .font_store
    .load_woff2_font(
      Path::new("assets/noto-sans-tc-v36-chinese-traditional_latin-regular.woff2"),
      "Noto Sans TC",
    )
    .unwrap();

  context
    .font_store
    .load_woff2_font(
      Path::new("assets/noto-sans-tc-v36-chinese-traditional_latin-700.woff2"),
      "Noto Sans TC 700",
    )
    .unwrap();

  // Initialize the router with our image generation endpoint
  let app = Router::new()
    .route("/image", post(generate_image_handler))
    .with_state(Arc::new(context));

  // Bind to all interfaces on port 3000
  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  let listener = TcpListener::bind(addr).await.unwrap();

  println!("Image generator server running on http://{}", addr);

  // Start the server
  axum::serve(listener, app).await.unwrap();
}
