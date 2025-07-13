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
  Query(query): Query<HmacQuery>,
  request: Request,
  next: Next,
) -> AxumResult<Response> {
  let Some(secret) = state.hmac_key.as_ref() else {
    return Ok(next.run(request).await);
  };

  if !verify_payload(&query, secret) {
    return Err((
      StatusCode::BAD_REQUEST,
      "HMAC verification failed".to_string(),
    ));
  }

  Ok(next.run(request).await)
}

fn verify_payload(query: &HmacQuery, secret: &[u8]) -> bool {
  let mut mac = Hmac::<Sha256>::new_from_slice(secret).unwrap();
  mac.update(query.payload.as_bytes());
  mac.update(b";");
  mac.update(query.timestamp.to_string().as_bytes());

  hex::decode(&query.hash)
    .ok()
    .and_then(|bytes| mac.verify_slice(&bytes).ok())
    .is_some()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_verify_payload() {
    let secret = b"secret";
    let query = HmacQuery {
      hash: "d3b19dff7172a21a942ebf543e1a1cc6b39e7b086313bc0c226352c512f36404".to_string(),
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };

    assert!(verify_payload(&query, secret));
  }
}
