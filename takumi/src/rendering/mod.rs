//! Rendering system for the takumi image generation library.
//!
//! This module contains all rendering-related functionality including:
//! - Canvas operations and image blending
//! - Drawing utilities for shapes and primitives
//! - Image renderer and viewport management
//! - Performance-optimized rendering operations

/// Canvas operations and image blending
pub mod canvas;
/// Drawing utilities and primitives
pub mod draw;
/// Main image renderer and viewport management
pub mod renderer;

pub use canvas::*;
pub use draw::*;
pub use renderer::*;
