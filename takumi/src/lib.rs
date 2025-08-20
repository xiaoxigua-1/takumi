#![doc(
  html_logo_url = "https://raw.githubusercontent.com/kane50613/takumi/master/assets/images/takumi.svg",
  html_favicon_url = "https://raw.githubusercontent.com/kane50613/takumi/master/assets/images/takumi.svg"
)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::use_self)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]

//! Takumi is a library with different parts to render your React components to images. This crate contains the core logic for layout, rendering.
//!
//! Checkout the [Getting Started](https://takumi.kane.tw/docs/getting-started) page if you are looking for Node.js / WASM bindings.
//!
//! # Walkthrough
//!
//! Everything starts with a [`ImageRenderer`](crate::rendering::ImageRenderer) instance, it takes [`Node`](crate::layout::node::Node) tree as input then calculate the layout.
//!
//! You can then draw the layout to an [`RgbaImage`](image::RgbaImage).
//!
//! # Example
//!
//! ```rust
//! use takumi::{
//!   layout::{
//!     node::{ContainerNode, TextNode, NodeKind, Node},
//!     Viewport,
//!     style::Style,
//!   },
//!   rendering::ImageRenderer,
//!   GlobalContext,
//! };
//!
//! // Create a node tree with `ContainerNode` and `TextNode`
//! let mut node = ContainerNode {
//!   children: Some(vec![
//!     TextNode {
//!       text: "Hello, world!".to_string(),
//!       style: Style::default(),
//!     }.into(),
//!   ]),
//!   style: Style::default(),
//! };
//!
//! // Inherit styles for children
//! node.inherit_style_for_children();
//!
//! // Create a context for storing resources, font caches.
//! // You should reuse the context to speed up the rendering.
//! let context = GlobalContext::default();
//!
//! // Load fonts
//! context.font_context.load_and_store(include_bytes!("../../assets/fonts/noto-sans/google-sans-code-v11-latin-regular.woff2").to_vec());
//!
//! // Create a renderer with a viewport
//! // You should create a new renderer for each render.
//! let mut renderer: ImageRenderer<NodeKind> = ImageRenderer::new(Viewport::new(1200, 630));
//!
//! // Construct the taffy tree, this will calculate the layout and store the result in the renderer.
//! renderer.construct_taffy_tree(node.into(), &context);
//!
//! // Draw the layout to an `RgbaImage`
//! let image = renderer.draw(&context).unwrap();
//! ```
//!
//! # Credits
//!
//! - [taffy](https://github.com/DioxusLabs/taffy) for the layout system.
//! - [image](https://github.com/image-rs/image) for the image processing.
//! - [cosmic-text](https://github.com/kornelski/cosmic-text) for the text rendering.
//! - [woff2-patched](https://github.com/zimond/woff2-rs) for the font processing.
//! - [ts-rs](https://github.com/AlephAlpha/ts-rs) for the type-safe serialization.
//! - [resvg](https://github.com/linebender/resvg) for SVG parsing and rendering.

/// Layout related modules, including the node tree, style parsing, and layout calculation.
pub mod layout;

/// Rendering related modules, including the image renderer, canvas operations.
pub mod rendering;

/// External resource management (fonts, images)
pub mod resources;

pub use image;
pub use taffy;

use crate::{
  rendering::RenderError,
  resources::{
    font::FontContext,
    image::{ImageResourceError, PersistentImageStore},
  },
};

/// The main context for image rendering.
///
/// This struct holds all the necessary state for rendering images, including
/// font management, image storage, and debug options.
#[derive(Default)]
pub struct GlobalContext {
  /// Whether to draw debug borders around nodes
  pub draw_debug_border: bool,
  /// The font context for text rendering
  pub font_context: FontContext,
  /// The image store for persisting contents
  pub persistent_image_store: PersistentImageStore,
}

/// Represents errors that can occur.
#[derive(Debug)]
pub enum Error {
  /// Represents an error that occurs during image resolution.
  ImageResolveError(ImageResourceError),
  /// Represents an error that occurs during rendering.
  RenderError(RenderError),
}
