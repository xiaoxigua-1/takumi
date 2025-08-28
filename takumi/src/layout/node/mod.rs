mod container;
mod image;
mod text;

pub use container::*;
pub use image::*;
pub use text::*;

use serde::{Deserialize, Serialize};
use taffy::{AvailableSpace, Layout, Point, Size};

use crate::{
  impl_node_enum,
  layout::style::Style,
  rendering::{
    BoxShadowRenderPhase, Canvas, RenderContext, draw_background_layers, draw_box_shadow,
  },
};

/// A trait representing a node in the layout tree.
///
/// This trait defines the common interface for all elements that can be
/// rendered in the layout system, including containers, text, and images.
pub trait Node<N: Node<N>>: Send + Sync + Clone {
  /// Return reference to children nodes.
  fn take_children(&mut self) -> Option<Vec<N>> {
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
  fn draw_on_canvas(&self, context: &RenderContext, canvas: &Canvas, layout: Layout) {
    self.draw_outset_box_shadow(context, canvas, layout);
    self.draw_background_color(context, canvas, layout);
    self.draw_background_image(context, canvas, layout);
    self.draw_inset_box_shadow(context, canvas, layout);
    self.draw_border(context, canvas, layout);
    self.draw_content(context, canvas, layout);
  }

  /// Draws the outset box shadow of the node.
  fn draw_outset_box_shadow(&self, context: &RenderContext, canvas: &Canvas, layout: Layout) {
    if let Some(box_shadow) = &self.get_style().box_shadow {
      let border_radius = self.get_style().create_border_radius(&layout, context);

      draw_box_shadow(
        context,
        box_shadow,
        border_radius,
        canvas,
        layout,
        BoxShadowRenderPhase::Outset,
      );
    }
  }

  /// Draws the inset box shadow of the node.
  fn draw_inset_box_shadow(&self, context: &RenderContext, canvas: &Canvas, layout: Layout) {
    if let Some(box_shadow) = &self.get_style().box_shadow {
      let border_radius = self.get_style().create_border_radius(&layout, context);

      draw_box_shadow(
        context,
        box_shadow,
        border_radius,
        canvas,
        layout,
        BoxShadowRenderPhase::Inset,
      );
    }
  }

  /// Draws the background color of the node.
  fn draw_background_color(&self, context: &RenderContext, canvas: &Canvas, layout: Layout) {
    if let Some(background_color) = &self.get_style().background_color {
      let radius = self.get_style().create_border_radius(&layout, context);

      canvas.fill_color(
        Point {
          x: layout.location.x as i32,
          y: layout.location.y as i32,
        },
        Size {
          width: layout.size.width as u32,
          height: layout.size.height as u32,
        },
        *background_color,
        radius,
      );
    }
  }

  /// Draws the background image(s) of the node.
  fn draw_background_image(&self, context: &RenderContext, canvas: &Canvas, layout: Layout) {
    draw_background_layers(self.get_style(), context, canvas, layout);
  }

  /// Draws the main content of the node.
  fn draw_content(&self, _context: &RenderContext, _canvas: &Canvas, _layout: Layout) {
    // Default implementation does nothing
  }

  /// Returns true if `draw_content` is needed to be called.
  fn has_draw_content(&self) -> bool {
    false
  }

  /// Draws the border of the node.
  fn draw_border(&self, context: &RenderContext, canvas: &Canvas, layout: Layout) {
    use crate::rendering::{BorderProperties, draw_border};

    let border = BorderProperties::from_layout(context, &layout, self.get_style());
    draw_border(canvas, border);
  }
}

/// Represents the nodes enum.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum NodeKind {
  /// A node that contains other nodes.
  Container(ContainerNode<NodeKind>),
  /// A node that displays an image.
  Image(ImageNode),
  /// A node that displays text.
  Text(TextNode),
}

impl_node_enum!(
  NodeKind,
  Container => ContainerNode<NodeKind>,
  Image => ImageNode,
  Text => TextNode
);
