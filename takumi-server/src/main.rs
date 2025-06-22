mod args;

use axum::{
  Router,
  extract::{Json, State},
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::post,
};
use clap::Parser;
use glob::glob;
use std::{fs::read, io::Cursor, net::SocketAddr, sync::Arc};
use takumi::{
  context::GlobalContext,
  image::ImageFormat,
  node::{DefaultNodeKind, Node, style::LengthUnit},
  render::{ImageRenderer, Viewport},
};
use tokio::{net::TcpListener, task::spawn_blocking};
use tracing::{Level, error, info};
use tracing_subscriber::fmt;

use mimalloc::MiMalloc;

use crate::args::Args;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

async fn generate_image_handler(
  State(context): State<Arc<GlobalContext>>,
  Json(mut root_node): Json<DefaultNodeKind>,
) -> Result<Response, StatusCode> {
  let LengthUnit::Px(width) = root_node.get_style().width else {
    return Err(StatusCode::BAD_REQUEST);
  };

  let LengthUnit::Px(height) = root_node.get_style().height else {
    return Err(StatusCode::BAD_REQUEST);
  };

  let buffer = spawn_blocking(move || -> Vec<u8> {
    root_node.inherit_style_for_children();
    root_node.hydrate(&context);

    let mut renderer = ImageRenderer::new(Viewport::new(width as u32, height as u32));

    renderer.construct_taffy_tree(root_node, &context);

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    let image = renderer.draw(&context).unwrap();

    image.write_to(&mut cursor, ImageFormat::WebP).unwrap();

    buffer
  })
  .await
  .unwrap();

  Ok(([("content-type", "image/webp")], buffer).into_response())
}

#[tokio::main]
async fn main() {
  fmt().with_max_level(Level::INFO).init();

  let args = Args::parse();

  let context = GlobalContext {
    print_debug_tree: args.print_debug_tree,
    draw_debug_border: args.draw_debug_border,
    ..Default::default()
  };

  if let Some(font_glob) = args.font_glob.as_ref() {
    for font in glob(font_glob).unwrap() {
      match font {
        Ok(path) => {
          if path.is_dir() {
            continue;
          }

          let file = read(&path).unwrap();

          context.font_context.load_font(file).unwrap();

          info!("Loaded font: {}", path.display())
        }
        Err(e) => error!("Failed to load font: {e}"),
      }
    }
  }

  // Initialize the router with our image generation endpoint
  let app = Router::new()
    .route("/image", post(generate_image_handler))
    .with_state(Arc::new(context));

  // Bind to all interfaces on port 3000
  let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
  let listener = TcpListener::bind(addr).await.unwrap();

  println!("Image generator server running on http://{}", addr);

  // Start the server
  axum::serve(listener, app).await.unwrap();
}
