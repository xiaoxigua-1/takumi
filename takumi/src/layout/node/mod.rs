mod container;
mod image;
mod text;

pub use container::*;
pub use image::*;
pub use text::*;

use serde::{Deserialize, Serialize};
use taffy::{AvailableSpace, Layout, Point, Size};

use crate::{
  layout::style::Style,
  rendering::{
    BorderProperties, BoxShadowRenderPhase, Canvas, RenderContext, draw_background_layers,
    draw_border, draw_box_shadow, resolve_layers_tiles,
  },
};

/// Implements the Node trait for an enum type that contains different node variants.
macro_rules! impl_node_enum {
  ($name:ident, $($variant:ident => $variant_type:ty),*) => {
    impl $crate::layout::node::Node<$name> for $name {
      fn take_children(&mut self) -> Option<Vec<$name>> {
        match self {
          $( $name::$variant(inner) => inner.take_children(), )*
        }
      }

      fn get_style(&self) -> &$crate::layout::style::Style {
        match self {
          $( $name::$variant(inner) => <_ as $crate::layout::node::Node<$name>>::get_style(inner), )*
        }
      }

      fn measure(
        &self,
        context: &$crate::rendering::RenderContext,
        available_space: $crate::taffy::Size<$crate::taffy::AvailableSpace>,
        known_dimensions: $crate::taffy::Size<Option<f32>>,
      ) -> $crate::taffy::Size<f32> {
        match self {
          $( $name::$variant(inner) => <_ as $crate::layout::node::Node<$name>>::measure(inner, context, available_space, known_dimensions), )*
        }
      }

      fn draw_on_canvas(&self, context: &$crate::rendering::RenderContext, canvas: &$crate::rendering::Canvas, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::layout::node::Node<$name>>::draw_on_canvas(inner, context, canvas, layout), )*
        }
      }

      fn draw_background_color(&self, context: &$crate::rendering::RenderContext, canvas: &$crate::rendering::Canvas, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::layout::node::Node<$name>>::draw_background_color(inner, context, canvas, layout), )*
        }
      }

      fn draw_content(&self, context: &$crate::rendering::RenderContext, canvas: &$crate::rendering::Canvas, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::layout::node::Node<$name>>::draw_content(inner, context, canvas, layout), )*
        }
      }

      fn draw_border(&self, context: &$crate::rendering::RenderContext, canvas: &$crate::rendering::Canvas, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::layout::node::Node<$name>>::draw_border(inner, context, canvas, layout), )*
        }
      }

      fn draw_outset_box_shadow(&self, context: &$crate::rendering::RenderContext, canvas: &$crate::rendering::Canvas, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::layout::node::Node<$name>>::draw_outset_box_shadow(inner, context, canvas, layout), )*
        }
      }

      fn draw_inset_box_shadow(&self, context: &$crate::rendering::RenderContext, canvas: &$crate::rendering::Canvas, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::layout::node::Node<$name>>::draw_inset_box_shadow(inner, context, canvas, layout), )*
        }
      }

      fn has_draw_content(&self) -> bool {
        match self {
          $( $name::$variant(inner) => <_ as $crate::layout::node::Node<$name>>::has_draw_content(inner), )*
        }
      }
    }

    $(
      impl From<$variant_type> for $name {
        fn from(inner: $variant_type) -> Self {
          $name::$variant(inner)
        }
      }
    )*
  };
}

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
    if let Some(box_shadow) = context.style.box_shadow.as_ref() {
      let border_radius = BorderProperties::from_context(context, &layout);

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
    if let Some(box_shadow) = context.style.box_shadow.as_ref() {
      let border_radius = BorderProperties::from_context(context, &layout);

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
    let radius = BorderProperties::from_context(context, &layout);

    canvas.fill_color(
      Point {
        x: layout.location.x as i32,
        y: layout.location.y as i32,
      },
      Size {
        width: layout.size.width as u32,
        height: layout.size.height as u32,
      },
      context.style.background_color,
      radius,
      context.transform,
    );
  }

  /// Draws the background image(s) of the node.
  fn draw_background_image(&self, context: &RenderContext, canvas: &Canvas, layout: Layout) {
    let Some(background_image) = context.style.background_image.as_ref() else {
      return;
    };

    let tiles = resolve_layers_tiles(
      background_image,
      context.style.background_position.as_ref(),
      context.style.background_size.as_ref(),
      context.style.background_repeat.as_ref(),
      context,
      layout,
    );

    draw_background_layers(
      tiles,
      BorderProperties::from_context(context, &layout).inset_by_border_width(),
      context,
      canvas,
      layout,
    );
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
    draw_border(
      canvas,
      layout.location,
      BorderProperties::from_context(context, &layout),
    );
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
