use std::io::Cursor;

use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::from_str;
use takumi::{
  DefaultNodeKind, ImageRenderer, LengthUnit, Node, Viewport,
  rendering::{ImageOutputFormat, write_image},
};
use tokio::task::spawn_blocking;

use crate::{AxumResult, AxumState};

#[derive(Deserialize)]
pub struct GenerateImageQuery {
  format: Option<ImageOutputFormat>,
  quality: Option<u8>,
  payload: String,
}

pub async fn generate_image_handler(
  Query(query): Query<GenerateImageQuery>,
  State(state): AxumState,
) -> AxumResult<Response> {
  let mut root_node: DefaultNodeKind = from_str(&query.payload).map_err(|err| {
    (
      StatusCode::BAD_REQUEST,
      format!("Failed to parse node: {err}"),
    )
  })?;

  let LengthUnit::Px(width) = root_node.get_style().width else {
    return Err((
      StatusCode::BAD_REQUEST,
      "Width must be specified in pixels".to_string(),
    ));
  };

  let LengthUnit::Px(height) = root_node.get_style().height else {
    return Err((
      StatusCode::BAD_REQUEST,
      "Height must be specified in pixels".to_string(),
    ));
  };

  let format = query.format.unwrap_or(ImageOutputFormat::WebP);

  let buffer = spawn_blocking(move || -> AxumResult<Vec<u8>> {
    root_node.inherit_style_for_children();
    root_node.hydrate(&state.context);

    let mut renderer = ImageRenderer::new(Viewport::new(width as u32, height as u32));

    renderer.construct_taffy_tree(root_node, &state.context);

    let image = renderer.draw(&state.context).map_err(|err| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to render image: {err}"),
      )
    })?;

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    write_image(&image, &mut cursor, format, query.quality).map_err(|err| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to write image: {err}"),
      )
    })?;

    Ok(buffer)
  })
  .await
  .map_err(|err| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("Image generation task panicked: {err}"),
    )
  })??;

  Ok(([("content-type", format.content_type())], buffer).into_response())
}

#[cfg(test)]
mod tests {
  use takumi::{ContainerNode, GlobalContext, Style};

  use crate::{args::Args, create_state};

  use super::*;

  #[tokio::test]
  async fn test_generate_image_handler() {
    let node: DefaultNodeKind = ContainerNode {
      style: Style {
        width: 100.0.into(),
        height: 100.0.into(),
        ..Default::default()
      },
      children: None,
    }
    .into();

    let state = create_state(Args::default(), GlobalContext::default());
    let response = generate_image_handler(
      Query(GenerateImageQuery {
        format: None,
        quality: None,
        payload: serde_json::to_string(&node).unwrap(),
      }),
      state,
    )
    .await
    .unwrap();
    assert_eq!(response.status(), 200);
  }

  #[tokio::test]
  async fn test_generate_image_handler_invalid_width() {
    let node: DefaultNodeKind = ContainerNode {
      style: Style::default(),
      children: None,
    }
    .into();

    let state = create_state(Args::default(), GlobalContext::default());
    let (status, _) = generate_image_handler(
      Query(GenerateImageQuery {
        format: None,
        quality: None,
        payload: serde_json::to_string(&node).unwrap(),
      }),
      state,
    )
    .await
    .unwrap_err();

    assert_eq!(status, 400);
  }
}
