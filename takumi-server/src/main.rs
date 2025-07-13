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

fn create_state(args: Args, context: GlobalContext) -> AxumState {
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

  State(state)
}

fn create_app(state: AxumState) -> Router {
  let mut app = Router::new()
    .route("/image", get(generate_image_handler))
    .with_state(state.0.clone());

  #[cfg(feature = "hmac_verify")]
  if state.hmac_key.is_some() {
    app = app.layer(axum::middleware::from_fn_with_state(
      state.0,
      hmac_verify::hmac_verify_middleware,
    ));
  }

  app
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

          if let Err(e) = context.font_context.load_font(file) {
            error!("Failed to load font {}: {e:?}", path.file_name().display());
            continue;
          }

          info!("Loaded font: {}", path.file_name().display())
        }
        Err(e) => error!("Failed to load font: {e:?}"),
      }
    }
  }

  let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
  let listener = TcpListener::bind(addr).await.unwrap();

  info!("Image generator server running on http://{addr}");

  axum::serve(listener, create_app(create_state(args, context)))
    .await
    .unwrap();
}

#[cfg(test)]
mod tests {
  use axum::http::Request;
  use tower::ServiceExt;

  use super::*;

  #[test]
  fn test_create_state() {
    let state = create_state(Args::default(), GlobalContext::default());
    assert!(!state.context.draw_debug_border);
    assert!(!state.context.print_debug_tree);
    #[cfg(feature = "hmac_verify")]
    assert!(state.hmac_key.is_none());
  }

  #[tokio::test]
  #[cfg(feature = "hmac_verify")]
  async fn test_generate_image_handler_with_hmac_verify() {
    use std::time::{SystemTime, UNIX_EPOCH};

    use axum::body::Body;
    use hmac::{Hmac, Mac};
    use query_string_builder::QueryString;
    use sha2::Sha256;
    use takumi::{ContainerNode, DefaultNodeKind, Style};

    let app = create_app(create_state(Args::default(), GlobalContext::default()));

    let node: DefaultNodeKind = ContainerNode {
      style: Style {
        width: 100.0.into(),
        height: 100.0.into(),
        ..Default::default()
      },
      children: None,
    }
    .into();

    let payload = serde_json::to_string(&node).unwrap();
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();

    let mut mac = Hmac::<Sha256>::new_from_slice(b"secret").unwrap();
    mac.update(payload.as_bytes());
    mac.update(b";");
    mac.update(timestamp.to_string().as_bytes());
    let hash = mac.finalize().into_bytes();

    let uri = QueryString::dynamic()
      .with_value("payload", payload)
      .with_value("timestamp", timestamp)
      .with_value("hash", hex::encode(hash));

    let request = Request::builder()
      .uri(format!("/image{uri}"))
      .body(Body::empty())
      .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(
      response.headers().get("content-type").unwrap(),
      "image/webp"
    );
  }
}
