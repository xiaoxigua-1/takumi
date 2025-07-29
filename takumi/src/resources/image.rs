//! Image resource management for the takumi rendering system.
//!
//! This module provides types and utilities for managing image resources,
//! including loading states, error handling, and image processing operations.

use image::RgbaImage;

/// Represents the state of an image resource.
pub type ImageState = Result<RgbaImage, ImageError>;

/// Represents the state of an image in the rendering system.
///
/// This enum tracks whether an image has been successfully loaded and decoded,
/// or if there was an error during the process.
#[derive(Debug)]
pub enum ImageError {
  /// An error occurred while fetching the image from the network
  NetworkError,
  /// An error occurred while decoding the image data
  DecodeError(image::ImageError),
  /// The image data URI is in an invalid format
  #[cfg(feature = "image_data_uri")]
  InvalidDataUriFormat,
  /// The image data URI is malformed and cannot be parsed
  MalformedDataUri,
  /// The image data URI feature is not enabled, so parsing is not supported
  #[cfg(not(feature = "image_data_uri"))]
  DataUriParseNotSupported,
}
