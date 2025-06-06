use axum::{
  Router,
  extract::{Json, State},
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::post,
};
use clap::Parser;
use image::ImageFormat;
use imagen::{context::Context, node::Node, render::ImageRenderer};
use std::{io::Cursor, net::SocketAddr, path::Path, sync::Arc};
use tokio::net::TcpListener;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value_t = false)]
  print_debug_tree: bool,

  #[arg(short, long, default_value_t = false)]
  draw_debug_border: bool,

  #[arg(short, long, value_parser)]
  fonts: Vec<String>,
}

async fn generate_image_handler(
  State(context): State<Arc<Context>>,
  Json(root_node): Json<Node>,
) -> Result<Response, StatusCode> {
  let renderer = ImageRenderer::try_from(root_node).map_err(|_| StatusCode::BAD_REQUEST)?;

  renderer.root_node.hydrate(context.as_ref()).await;

  let (mut taffy, root_node_id) = renderer.create_taffy_tree();

  let mut buffer = Vec::new();
  let mut cursor = Cursor::new(&mut buffer);

  let image = renderer.draw(context.as_ref(), &mut taffy, root_node_id);

  image.write_to(&mut cursor, ImageFormat::WebP).unwrap();

  Ok(([("content-type", "image/webp")], buffer).into_response())
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let context = Context {
    print_debug_tree: args.print_debug_tree,
    draw_debug_border: args.draw_debug_border,
    ..Default::default()
  };

  context
    .load_woff2_font(Path::new(
      "assets/noto-sans-tc-v36-chinese-traditional_latin-regular.woff2",
    ))
    .unwrap();

  context
    .load_woff2_font(Path::new(
      "assets/noto-sans-tc-v36-chinese-traditional_latin-700.woff2",
    ))
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
