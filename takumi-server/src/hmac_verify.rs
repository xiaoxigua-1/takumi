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

  if let Err(err) = verify_payload(&query, secret) {
    return Err((StatusCode::BAD_REQUEST, err.to_string()));
  }

  Ok(next.run(request).await)
}

const ERROR_HASH_LENGTH: &str = "Hash must be a 64-character hexadecimal string";
const ERROR_INVALID_HEX: &str = "Invalid hexadecimal hash";
const ERROR_HMAC_VERIFICATION: &str = "HMAC verification failed";

fn verify_payload(query: &HmacQuery, secret: &[u8]) -> Result<(), &'static str> {
  if query.hash.len() != 64 {
    return Err(ERROR_HASH_LENGTH);
  }

  let decoded_hash = hex::decode(&query.hash).map_err(|_| ERROR_INVALID_HEX)?;

  let mut mac = Hmac::<Sha256>::new_from_slice(secret).unwrap();
  mac.update(query.payload.as_bytes());
  mac.update(b";");
  mac.update(query.timestamp.to_string().as_bytes());

  mac
    .verify_slice(&decoded_hash)
    .map_err(|_| ERROR_HMAC_VERIFICATION)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_verify_payload_success() {
    let secret = b"secret";
    let query = HmacQuery {
      hash: "d3b19dff7172a21a942ebf543e1a1cc6b39e7b086313bc0c226352c512f36404".to_string(),
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };

    assert!(verify_payload(&query, secret).is_ok());
  }

  #[test]
  fn test_verify_payload_invalid_hash() {
    let secret = b"secret";
    let query = HmacQuery {
      hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };

    assert!(verify_payload(&query, secret).is_err());
    assert_eq!(
      verify_payload(&query, secret).unwrap_err(),
      ERROR_HMAC_VERIFICATION
    );
  }

  #[test]
  fn test_verify_payload_wrong_secret() {
    let secret = b"wrong_secret";
    let query = HmacQuery {
      hash: "d3b19dff7172a21a942ebf543e1a1cc6b39e7b086313bc0c226352c512f36404".to_string(),
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };

    assert!(verify_payload(&query, secret).is_err());
    assert_eq!(
      verify_payload(&query, secret).unwrap_err(),
      ERROR_HMAC_VERIFICATION
    );
  }

  #[test]
  fn test_verify_payload_malformed_hex() {
    let secret = b"secret";
    let query = HmacQuery {
      hash: "not-a-valid-hex-string".to_string(),
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };

    assert!(verify_payload(&query, secret).is_err());
    assert_eq!(
      verify_payload(&query, secret).unwrap_err(),
      ERROR_HASH_LENGTH
    );
  }

  #[test]
  fn test_verify_payload_empty_secret() {
    let secret = b"";
    let mut mac = Hmac::<Sha256>::new_from_slice(secret).unwrap();
    mac.update(b"payload");
    mac.update(b";");
    mac.update(b"1672531200");
    let result = mac.finalize();
    let expected_hash = hex::encode(result.into_bytes());

    let query = HmacQuery {
      hash: expected_hash,
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };

    assert!(verify_payload(&query, secret).is_ok());
  }

  #[test]
  fn test_verify_payload_empty_payload() {
    let secret = b"secret";
    let mut mac = Hmac::<Sha256>::new_from_slice(secret).unwrap();
    mac.update(b"");
    mac.update(b";");
    mac.update(b"1672531200");
    let result = mac.finalize();
    let expected_hash = hex::encode(result.into_bytes());

    let query = HmacQuery {
      hash: expected_hash,
      timestamp: 1672531200,
      payload: "".to_string(),
    };

    assert!(verify_payload(&query, secret).is_ok());
  }

  #[test]
  fn test_verify_payload_different_timestamp() {
    let secret = b"secret";
    let query = HmacQuery {
      hash: "d3b19dff7172a21a942ebf543e1a1cc6b39e7b086313bc0c226352c512f36404".to_string(),
      timestamp: 1672531201, // Different timestamp
      payload: "payload".to_string(),
    };

    assert!(verify_payload(&query, secret).is_err());
    assert_eq!(
      verify_payload(&query, secret).unwrap_err(),
      ERROR_HMAC_VERIFICATION
    );
  }

  #[test]
  fn test_verify_payload_short_hash() {
    let secret = b"secret";
    let query = HmacQuery {
      hash: "abcd".to_string(), // Too short
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };

    assert!(verify_payload(&query, secret).is_err());
    assert_eq!(
      verify_payload(&query, secret).unwrap_err(),
      ERROR_HASH_LENGTH
    );
  }

  #[test]
  fn test_verify_payload_case_sensitivity() {
    let secret = b"secret";
    let query = HmacQuery {
      hash: "D3B19DFF7172A21A942EBF543E1A1CC6B39E7B086313BC0C226352C512F36404".to_string(), // Uppercase
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };

    assert!(verify_payload(&query, secret).is_ok());
  }

  #[test]
  fn test_verify_payload_hash_length_validation() {
    let secret = b"secret";

    // Test 63 characters (too short)
    let query_short = HmacQuery {
      hash: "a".repeat(63),
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };
    assert!(verify_payload(&query_short, secret).is_err());
    assert_eq!(
      verify_payload(&query_short, secret).unwrap_err(),
      ERROR_HASH_LENGTH
    );

    // Test 65 characters (too long)
    let query_long = HmacQuery {
      hash: "a".repeat(65),
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };
    assert!(verify_payload(&query_long, secret).is_err());
    assert_eq!(
      verify_payload(&query_long, secret).unwrap_err(),
      ERROR_HASH_LENGTH
    );
  }

  #[test]
  fn test_verify_payload_non_hex_characters() {
    let secret = b"secret";
    let query = HmacQuery {
      hash: "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string(), // Invalid hex chars
      timestamp: 1672531200,
      payload: "payload".to_string(),
    };

    assert!(verify_payload(&query, secret).is_err());
    assert_eq!(
      verify_payload(&query, secret).unwrap_err(),
      ERROR_HASH_LENGTH
    );
  }
}
