/// Module for border drawing operations
pub mod border;
/// Module for drawing operations on canvas
pub mod draw;
/// Module for measuring text and image dimensions
pub mod measure;
/// Module for styling and layout properties
pub mod style;

use std::fmt::Debug;
use std::sync::{Arc, OnceLock};

use async_trait::async_trait;
use dyn_clone::{DynClone, clone_trait_object};
use futures_util::future::join_all;
use image::RgbaImage;
use imageproc::drawing::Blend;
use merge::Merge;
use serde::{Deserialize, Serialize};
use taffy::{AvailableSpace, Layout, NodeId, Size, TaffyError};

use crate::node::draw::draw_background_color;
use crate::{
  context::Context,
  node::{
    border::draw_border,
    draw::{ImageState, draw_image, draw_text},
    measure::{measure_image, measure_text},
    style::Style,
  },
  render::TaffyTreeWithNodes,
};

/// A trait representing a node in the layout tree.
///
/// This trait defines the common interface for all elements that can be
/// rendered in the layout system, including containers, text, and images.
#[typetag::serde(tag = "type")]
#[async_trait]
pub trait Node: Send + Sync + Debug + DynClone {
  /// Returns a reference to the node's style properties.
  fn get_style(&self) -> &Style;

  /// Returns a mutable reference to the node's style properties.
  fn get_style_mut(&mut self) -> &mut Style;

  /// Inherits style properties from a parent node.
  ///
  /// This method merges inheritable style properties from the parent
  /// into this node's style, then propagates the inheritance to children.
  fn inherit_style(&mut self, parent: &Style) {
    let style = self.get_style_mut();

    style
      .inheritable_style
      .merge(parent.inheritable_style.clone());

    self.inherit_style_for_children();
  }

  /// Propagates style inheritance to child nodes.
  ///
  /// Override this method in container nodes to pass styles to children.
  fn inherit_style_for_children(&mut self) {}

  /// Returns true if this node requires async hydration before rendering.
  ///
  /// Used for nodes that need to load external resources like images.
  fn should_hydrate_async(&self) -> bool {
    false
  }

  /// Performs async hydration of the node.
  ///
  /// This method is called for nodes that return true from `should_hydrate_async()`
  /// to load external resources before rendering.
  async fn hydrate_async(&self, _context: &Context) {}

  /// Measures the intrinsic size of the node.
  ///
  /// This method calculates the size the node would prefer given
  /// the available space and any known dimensions.
  ///
  /// # Arguments
  /// * `_context` - The rendering context
  /// * `_available_space` - The space available for this node
  /// * `_known_dimensions` - Any explicitly set dimensions
  ///
  /// # Returns
  /// The preferred size of the node
  fn measure(
    &self,
    _context: &Context,
    _available_space: Size<AvailableSpace>,
    _known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    Size::ZERO
  }

  /// Draws the node onto the canvas using the computed layout.
  ///
  /// This method orchestrates the drawing of background, content, and borders.
  ///
  /// # Arguments
  /// * `context` - The rendering context
  /// * `canvas` - The canvas to draw on
  /// * `layout` - The computed layout information for this node
  fn draw_on_canvas(&self, context: &Context, canvas: &mut Blend<RgbaImage>, layout: Layout) {
    self.draw_background(context, canvas, layout);
    self.draw_content(context, canvas, layout);
    self.draw_border(context, canvas, layout);
  }

  /// Draws the background of the node.
  ///
  /// # Arguments
  /// * `_context` - The rendering context
  /// * `canvas` - The canvas to draw on
  /// * `layout` - The computed layout information for this node
  fn draw_background(&self, _context: &Context, canvas: &mut Blend<RgbaImage>, layout: Layout) {
    if let Some(background_color) = self.get_style().background_color {
      draw_background_color(background_color, canvas, layout);
    }
  }

  /// Draws the main content of the node.
  ///
  /// This method should be overridden by specific node types to draw their content.
  ///
  /// # Arguments
  /// * `_context` - The rendering context
  /// * `_canvas` - The canvas to draw on
  /// * `_layout` - The computed layout information for this node
  fn draw_content(&self, _context: &Context, _canvas: &mut Blend<RgbaImage>, _layout: Layout) {
    // Default implementation does nothing
  }

  /// Draws the border of the node.
  ///
  /// # Arguments
  /// * `_context` - The rendering context
  /// * `canvas` - The canvas to draw on
  /// * `layout` - The computed layout information for this node
  fn draw_border(&self, _context: &Context, canvas: &mut Blend<RgbaImage>, layout: Layout) {
    draw_border(self.get_style(), canvas, &layout);
  }

  /// Creates a Taffy layout node for this element.
  ///
  /// This method integrates the node into the Taffy layout system,
  /// converting the node's style properties to Taffy's format.
  ///
  /// # Arguments
  /// * `taffy` - The Taffy tree to add this node to
  ///
  /// # Returns
  /// The ID of the created node in the Taffy tree
  fn create_taffy_leaf(&self, taffy: &mut TaffyTreeWithNodes) -> Result<NodeId, TaffyError>;
}

clone_trait_object!(Node);

/// A container node that can hold child nodes.
///
/// Container nodes are used to group other nodes and apply layout
/// properties like flexbox layout to arrange their children.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ContainerNode {
  /// The styling properties for this container
  #[serde(default, flatten)]
  pub style: Style,
  /// The child nodes contained within this container
  pub children: Vec<Box<dyn Node>>,
}

#[async_trait]
#[typetag::serde(name = "container")]
impl Node for ContainerNode {
  fn get_style(&self) -> &Style {
    &self.style
  }

  fn get_style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

  fn should_hydrate_async(&self) -> bool {
    self
      .children
      .iter()
      .any(|child| child.should_hydrate_async())
  }

  async fn hydrate_async(&self, context: &Context) {
    let futures = self.children.iter().filter_map(|child| {
      if child.should_hydrate_async() {
        Some(child.hydrate_async(context))
      } else {
        None
      }
    });

    join_all(futures).await;
  }

  fn inherit_style_for_children(&mut self) {
    let style = self.get_style().clone();

    for child in self.children.iter_mut() {
      child.inherit_style(&style);
    }
  }

  fn create_taffy_leaf(&self, taffy: &mut TaffyTreeWithNodes) -> Result<NodeId, TaffyError> {
    let taffy_style = self.style.clone().into();

    let children = self
      .children
      .iter()
      .map(|child| child.create_taffy_leaf(taffy))
      .collect::<Result<Vec<NodeId>, TaffyError>>()?;

    let container = taffy.new_with_children(taffy_style, children.as_slice())?;

    taffy.set_node_context(
      container,
      Some(Box::new(ContainerNode {
        style: self.style.clone(),
        children: vec![],
      })),
    )?;

    Ok(container)
  }
}

/// A node that renders text content.
///
/// Text nodes display text with configurable font properties,
/// alignment, and styling options.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextNode {
  /// The styling properties for this text node
  #[serde(default, flatten)]
  pub style: Style,
  /// The text content to be rendered
  pub text: String,
}

#[typetag::serde(name = "text")]
impl Node for TextNode {
  fn get_style(&self) -> &Style {
    &self.style
  }

  fn get_style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

  fn create_taffy_leaf(&self, taffy: &mut TaffyTreeWithNodes) -> Result<NodeId, TaffyError> {
    taffy.new_leaf_with_context(self.style.clone().into(), Box::new(self.clone()))
  }

  fn draw_content(&self, context: &Context, canvas: &mut Blend<RgbaImage>, layout: Layout) {
    draw_text(
      &self.text,
      &(&self.style).into(),
      &context.font_context,
      canvas,
      layout,
    );
  }

  fn measure(
    &self,
    context: &Context,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    measure_text(
      &context.font_context,
      &self.text,
      &(&self.style).into(),
      known_dimensions,
      available_space,
    )
  }
}

/// A node that renders image content.
///
/// Image nodes display images loaded from URLs or file paths,
/// with support for async loading and caching.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageNode {
  /// The styling properties for this image node
  #[serde(default, flatten)]
  pub style: Style,
  /// The source URL or path to the image
  pub src: String,
  /// The cached image state (not serialized)
  #[serde(skip)]
  pub image: Arc<OnceLock<Arc<ImageState>>>,
}

#[typetag::serde(name = "image")]
#[async_trait]
impl Node for ImageNode {
  fn get_style(&self) -> &Style {
    &self.style
  }

  fn get_style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

  fn create_taffy_leaf(&self, taffy: &mut TaffyTreeWithNodes) -> Result<NodeId, TaffyError> {
    taffy.new_leaf_with_context(self.style.clone().into(), Box::new(self.clone()))
  }

  fn should_hydrate_async(&self) -> bool {
    self.image.get().is_none()
  }

  async fn hydrate_async(&self, context: &Context) {
    let image_store = &context.image_store;

    if let Some(img) = image_store.get(&self.src) {
      self.image.set(img).unwrap();
      return;
    }

    let img = image_store.fetch_async(&self.src).await;

    image_store.insert(self.src.clone(), img.clone());
    self.image.set(img).unwrap();
  }

  fn measure(
    &self,
    _context: &Context,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    let ImageState::Fetched(image) = self.image.get().unwrap().as_ref() else {
      return Size::ZERO;
    };

    let (width, height) = image.dimensions();

    measure_image(
      Size {
        width: width as f32,
        height: height as f32,
      },
      known_dimensions,
      available_space,
    )
  }

  fn draw_content(&self, _context: &Context, canvas: &mut Blend<RgbaImage>, layout: Layout) {
    let ImageState::Fetched(image) = self.image.get().unwrap().as_ref() else {
      return;
    };

    draw_image(image, &self.style, canvas, layout);
  }
}
