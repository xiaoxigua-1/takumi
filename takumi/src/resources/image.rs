//! Image resource management for the takumi rendering system.
//!
//! This module provides types and utilities for managing image resources,
//! including loading states, error handling, and image processing operations.

use image::{
  RgbaImage,
  imageops::{FilterType, resize},
};

/// Represents the state of an image resource.
pub type ImageResult = Result<ImageSource, ImageError>;

#[derive(Debug, Clone)]
/// Represents the source of an image.
pub enum ImageSource {
  /// An svg image source
  #[cfg(feature = "svg")]
  Svg(Box<resvg::usvg::Tree>),
  /// A bitmap image source
  Bitmap(RgbaImage),
}

impl From<RgbaImage> for ImageSource {
  fn from(bitmap: RgbaImage) -> Self {
    ImageSource::Bitmap(bitmap)
  }
}

impl ImageSource {
  /// Get the size of the image source.
  pub fn size(&self) -> (f32, f32) {
    match self {
      #[cfg(feature = "svg")]
      ImageSource::Svg(svg) => (svg.size().width(), svg.size().height()),
      ImageSource::Bitmap(bitmap) => (bitmap.width() as f32, bitmap.height() as f32),
    }
  }

  /// Render the image source to an RGBA image with the specified dimensions.
  pub fn render_to_rgba_image(&self, width: u32, height: u32) -> RgbaImage {
    match self {
      ImageSource::Bitmap(bitmap) => resize(bitmap, width, height, FilterType::Lanczos3),
      #[cfg(feature = "svg")]
      ImageSource::Svg(svg) => {
        use resvg::{tiny_skia::Pixmap, usvg::Transform};

        let mut pixmap = Pixmap::new(width, height).unwrap();

        let original_size = svg.size();
        let sx = width as f32 / original_size.width();
        let sy = height as f32 / original_size.height();

        resvg::render(svg, Transform::from_scale(sx, sy), &mut pixmap.as_mut());

        RgbaImage::from_raw(width, height, pixmap.take()).unwrap()
      }
    }
  }
}

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
  #[cfg(feature = "svg")]
  /// An error occurred while parsing an SVG image
  SvgParseError(resvg::usvg::Error),
  /// SVG parsing is not supported in this build
  #[cfg(not(feature = "svg"))]
  SvgParseNotSupported,
  /// Remote image store is not available
  RemoteStoreNotAvailable,
}
