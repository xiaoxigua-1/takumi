use axum::{
  extract::{Query, Request, State},
  http::StatusCode,
  middleware::Next,
  response::Response,
};
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;

use crate::{AxumResult, AxumState};

#[derive(Deserialize)]
pub struct HmacQuery {
  pub hash: String,
  pub timestamp: u64,
  pub payload: String,
}

pub async fn hmac_verify_middleware(
  State(state): AxumState,
  Query(mut query): Query<HmacQuery>,
  request: Request,
  next: Next,
) -> AxumResult<Response> {
  let Some(secret) = state.hmac_key.as_ref() else {
    return Ok(next.run(request).await);
  };

  query.payload.push_str(&query.timestamp.to_string());

  let mut mac = Hmac::<Sha256>::new_from_slice(secret).unwrap();
  mac.update(query.payload.as_bytes());

  let result = mac.finalize();
  let result_bytes = result.into_bytes();
  let result_hex = hex::encode(result_bytes);

  if result_hex != query.hash {
    return Err((
      StatusCode::BAD_REQUEST,
      "HMAC verification failed".to_string(),
    ));
  }

  Ok(next.run(request).await)
}
