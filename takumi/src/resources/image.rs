//! Image resource management for the takumi rendering system.
//!
//! This module provides types and utilities for managing image resources,
//! including loading states, error handling, and image processing operations.

use image::{ImageError, RgbaImage};

/// Represents the state of an image in the rendering system.
///
/// This enum tracks whether an image has been successfully loaded and decoded,
/// or if there was an error during the process.
#[derive(Debug)]
pub enum ImageState {
  /// The image has been successfully loaded and decoded
  Fetched(RgbaImage),
  /// An error occurred while fetching the image from the network
  NetworkError,
  /// An error occurred while decoding the image data
  DecodeError(ImageError),
}

impl ImageState {
  /// Check if image is in fetched state
  pub fn is_fetched(&self) -> bool {
    matches!(self, ImageState::Fetched(_))
  }

  /// Get the image if it's in the fetched state
  pub fn as_image(&self) -> Option<&RgbaImage> {
    match self {
      ImageState::Fetched(image) => Some(image),
      _ => None,
    }
  }

  /// Check if the image state represents an error
  pub fn is_error(&self) -> bool {
    matches!(self, ImageState::NetworkError | ImageState::DecodeError(_))
  }

  /// Get error information if the state represents an error
  pub fn error_info(&self) -> Option<String> {
    match self {
      ImageState::NetworkError => Some("Network error occurred while fetching image".to_string()),
      ImageState::DecodeError(err) => Some(format!("Image decode error: {}", err)),
      ImageState::Fetched(_) => None,
    }
  }
}
