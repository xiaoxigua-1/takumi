mod args;

mod generate_image;
#[cfg(feature = "hmac_verify")]
mod hmac_verify;

use axum::{Router, extract::State, http::StatusCode, response::Response, routing::get};
use clap::Parser;

use globwalk::glob;
use std::{fs::read, net::SocketAddr, sync::Arc};
use takumi::GlobalContext;
use tokio::net::TcpListener;
use tracing::{Level, error, info};
use tracing_subscriber::fmt;

use mimalloc::MiMalloc;

use crate::{args::Args, generate_image::generate_image_handler};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub type AxumState = State<Arc<AxumStateInner>>;

pub type AxumResult<T = Response> = Result<T, (StatusCode, String)>;

pub struct AxumStateInner {
  pub context: GlobalContext,
  #[cfg(feature = "hmac_verify")]
  pub hmac_key: Option<Vec<u8>>,
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
          if path.path().is_dir() {
            continue;
          }

          let file = read(path.path()).unwrap();

          context.font_context.load_font(file).unwrap();

          info!("Loaded font: {}", path.file_name().display())
        }
        Err(e) => error!("Failed to load font: {e}"),
      }
    }
  }

  let state = Arc::new(AxumStateInner {
    context,
    #[cfg(feature = "hmac_verify")]
    hmac_key: args.hmac_key.map(|key| {
      use sha2::{Digest, Sha256};

      let mut hasher = Sha256::new();
      hasher.update(key.as_bytes());
      hasher.finalize().to_vec()
    }),
  });

  let mut app = Router::new()
    .route("/image", get(generate_image_handler))
    .with_state(state.clone());

  #[cfg(feature = "hmac_verify")]
  if state.hmac_key.is_some() {
    app = app.layer(axum::middleware::from_fn_with_state(
      state,
      hmac_verify::hmac_verify_middleware,
    ));
  }

  let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
  let listener = TcpListener::bind(addr).await.unwrap();

  println!("Image generator server running on http://{addr}");

  axum::serve(listener, app).await.unwrap();
}
