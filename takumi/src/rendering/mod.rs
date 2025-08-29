/// Background and color drawing functions
mod background_drawing;
/// Canvas operations and image blending
mod canvas;
mod components;
/// Debug drawing utilities
mod debug_drawing;
/// Image drawing functions
mod image_drawing;
/// Main image renderer and viewport management
mod render;
/// Text drawing functions
mod text_drawing;

pub use background_drawing::*;
pub use canvas::*;
pub use components::*;
pub use debug_drawing::*;
pub use image_drawing::*;
pub use render::*;
use taffy::Size;
pub use text_drawing::*;

use crate::{GlobalContext, layout::Viewport};

const DEFAULT_SCALE_FACTOR: f32 = 1.0;

/// The default scale factor for the image renderer.
pub const DEFAULT_SCALE: Size<f32> = Size {
  width: DEFAULT_SCALE_FACTOR,
  height: DEFAULT_SCALE_FACTOR,
};

/// The context for the image renderer.
#[derive(Clone, Copy)]
pub struct RenderContext<'g> {
  /// The global context.
  pub global: &'g GlobalContext,
  /// The viewport for the image renderer.
  pub viewport: Viewport,
  /// The font size in pixels, used for em and rem units.
  pub parent_font_size: f32,
  /// The scale factor for the image renderer.
  pub scale: Size<f32>,
}
