use axum::extract::Query;
use takumi::DefaultNodeKind;

use takumi::{ContainerNode, GlobalContext, Style};
use takumi_server::{GenerateImageQuery, generate_image_handler};
use takumi_server::{args::Args, create_state};

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
