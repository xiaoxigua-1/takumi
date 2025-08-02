//! Core Node trait definition for the takumi layout system.
//!
//! This module contains the Node trait that defines the interface for
//! all renderable elements in the layout system.

use std::fmt::Debug;

use taffy::{AvailableSpace, Layout, Size};

use crate::{
  ColorInput,
  core::{GlobalContext, RenderContext},
  effects::{BorderRadius, draw_box_shadow},
  rendering::{FastBlendImage, draw_background},
  style::Style,
};

/// A trait representing a node in the layout tree.
///
/// This trait defines the common interface for all elements that can be
/// rendered in the layout system, including containers, text, and images.
pub trait Node<N: Node<N>>: Send + Sync + Debug + Clone {
  /// Return reference to children nodes.
  fn get_children(&self) -> Option<Vec<&N>> {
    None
  }

  /// Returns a reference to the node's style properties.
  fn get_style(&self) -> &Style;

  /// Returns a mutable reference to the node's style properties.
  fn get_style_mut(&mut self) -> &mut Style;

  /// Inherits style properties from a parent node.
  ///
  /// This method merges inheritable style properties from the parent
  /// into this node's style, then propagates the inheritance to children.
  fn inherit_style(&mut self, parent: &Style) {
    use merge::Merge;

    let style = self.get_style_mut();

    style
      .inheritable_style
      .merge(parent.inheritable_style.clone());

    self.inherit_style_for_children();
  }

  /// Called after the style is inherited and before the layout is computed.
  ///
  /// You can use this method to modify the node's style before the layout is computed.
  fn before_layout(&mut self) {}

  /// Propagates style inheritance to child nodes.
  ///
  /// Override this method in container nodes to pass styles to children.
  fn inherit_style_for_children(&mut self) {}

  /// Returns true if this node requires hydration before rendering.
  ///
  /// Used for nodes that need to load external resources like images.
  fn should_hydrate(&self) -> bool {
    false
  }

  /// Performs hydration of the node.
  ///
  /// This method is called for nodes that return true from `should_hydrate()`
  /// to load external resources before rendering.
  fn hydrate(&self, _context: &GlobalContext) -> Result<(), crate::Error> {
    Ok(())
  }

  /// Measures the intrinsic size of the node.
  ///
  /// This method calculates the size the node would prefer given
  /// the available space and any known dimensions.
  fn measure(
    &self,
    _context: &RenderContext,
    _available_space: Size<AvailableSpace>,
    _known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    Size::ZERO
  }

  /// Draws the node onto the canvas using the computed layout.
  fn draw_on_canvas(&self, context: &RenderContext, canvas: &mut FastBlendImage, layout: Layout) {
    self.draw_background_color(context, canvas, layout);
    self.draw_background_image(context, canvas, layout);
    self.draw_border(context, canvas, layout);
    self.draw_content(context, canvas, layout);
    self.draw_box_shadow(context, canvas, layout);
  }

  /// Draws the box shadow of the node.
  fn draw_box_shadow(&self, context: &RenderContext, canvas: &mut FastBlendImage, layout: Layout) {
    if let Some(box_shadow) = &self.get_style().box_shadow {
      let border_radius = self
        .get_style()
        .inheritable_style
        .border_radius
        .map(|radius| BorderRadius::from_layout(context, &layout, radius.into()));

      draw_box_shadow(context, box_shadow, border_radius, canvas, layout);
    }
  }

  /// Draws the background color of the node.
  fn draw_background_color(
    &self,
    context: &RenderContext,
    canvas: &mut FastBlendImage,
    layout: Layout,
  ) {
    if let Some(background_color) = &self.get_style().background_color {
      let radius = self
        .get_style()
        .inheritable_style
        .border_radius
        .map(|radius| BorderRadius::from_layout(context, &layout, radius.into()));

      draw_background(
        &ColorInput::Color(*background_color),
        radius,
        canvas,
        layout,
      );
    }
  }

  /// Draws the background image(s) of the node.
  fn draw_background_image(
    &self,
    context: &RenderContext,
    canvas: &mut FastBlendImage,
    layout: Layout,
  ) {
    if let Some(background_image) = &self.get_style().background_image {
      let radius = self
        .get_style()
        .inheritable_style
        .border_radius
        .map(|radius| BorderRadius::from_layout(context, &layout, radius.into()));

      draw_background(
        &ColorInput::Gradient(background_image.clone()),
        radius,
        canvas,
        layout,
      );
    }
  }

  /// Draws the main content of the node.
  fn draw_content(&self, _context: &RenderContext, _canvas: &mut FastBlendImage, _layout: Layout) {
    // Default implementation does nothing
  }

  /// Draws the border of the node.
  fn draw_border(&self, context: &RenderContext, canvas: &mut FastBlendImage, layout: Layout) {
    use crate::effects::{BorderProperties, draw_border};

    let border = BorderProperties::from_layout(context, &layout, self.get_style());
    draw_border(canvas, border);
  }
}
