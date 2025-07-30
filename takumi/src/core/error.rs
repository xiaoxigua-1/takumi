use crate::resources::ImageError;

/// Represents errors that can occur.
#[derive(Debug)]
pub enum Error {
  /// Represents an error that occurs during image resolution.
  ImageResolveError(ImageError),
}

impl From<ImageError> for Error {
  fn from(error: ImageError) -> Self {
    Error::ImageResolveError(error)
  }
}
