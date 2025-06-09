mod image_store;

use axum::{
  Router,
  extract::{Json, State},
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::post,
};
use clap::Parser;
use imagen::{
  context::{Context, load_woff2_font_to_context},
  image::ImageFormat,
  node::{ContainerNode, Node},
  render::ImageRenderer,
};
use reqwest::Client;
use std::{io::Cursor, net::SocketAddr, path::Path, sync::Arc};
use tokio::net::TcpListener;

use mimalloc::MiMalloc;

use crate::image_store::ImageStore;

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
  Json(root_node): Json<ContainerNode>,
) -> Result<Response, StatusCode> {
  let mut renderer = ImageRenderer::try_from(root_node).map_err(|_| StatusCode::BAD_REQUEST)?;

  renderer.root_node.inherit_style_for_children();
  renderer.root_node.hydrate_async(&context).await;

  let (mut taffy, root_node_id) = renderer.create_taffy_tree();

  let mut buffer = Vec::new();
  let mut cursor = Cursor::new(&mut buffer);

  let image = renderer.draw(&context, &mut taffy, root_node_id);

  image.write_to(&mut cursor, ImageFormat::WebP).unwrap();

  Ok(([("content-type", "image/webp")], buffer).into_response())
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let context = Context {
    print_debug_tree: args.print_debug_tree,
    draw_debug_border: args.draw_debug_border,
    image_store: Box::new(ImageStore::new(Client::new())),
    ..Default::default()
  };

  for font in args.fonts {
    load_woff2_font_to_context(&context.font_context, Path::new(&font)).unwrap();
  }

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
