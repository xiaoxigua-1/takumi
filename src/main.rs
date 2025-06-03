use axum::{
  Router,
  extract::Json,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::post,
};
use imagen::{color::Color, node::NodeKind};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct ImageRequest {
  pub width: u32,
  pub height: u32,
  pub background_color: Option<Color>,
  pub nodes: Vec<NodeKind>,
}

async fn generate_image_handler(Json(request): Json<ImageRequest>) -> Result<Response, StatusCode> {
  match generator.generate_image(request).await {
    Ok(image_bytes) => Ok(([("content-type", "image/png")], image_bytes).into_response()),
    Err(e) => {
      eprintln!("Error generating image: {}", e);
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

#[tokio::main]
async fn main() {
  // Initialize the router with our image generation endpoint
  let app = Router::new().route("/image", post(generate_image_handler));

  // Bind to all interfaces on port 3000
  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  let listener = TcpListener::bind(addr).await.unwrap();

  println!("Image generator server running on http://{}", addr);

  // Start the server
  axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_basic_generation() {
    let request = ImageRequest {
      nodes: vec![Node {
        node_type: NodeType::Rect(RectNode {
          width: 100.0,
          height: 50.0,
          color: Some(Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
          }),
        }),
        position: None,
        flex_style: None,
        children: None,
      }],
      width: 800,
      height: 600,
      background_color: Some(Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
      }),
    };

    let generator = ImageGenerator::new();
    let result = generator.generate_image(request).await;
    assert!(result.is_ok());
  }
}
