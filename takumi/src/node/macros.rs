/// Implements the Node trait for an enum type that contains different node variants.
///
/// This macro is used to implement the Node trait for enum types that represent
/// different kinds of nodes in the layout system. It delegates all Node trait
/// methods to the inner node contained in each variant.
///
/// # Example
/// ```rust
/// use takumi::{impl_node_enum, node::{ContainerNode, ImageNode, TextNode}};
///
/// #[derive(Debug, Clone)]
/// enum NodeKind {
///   Image(ImageNode),
///   Text(TextNode),
///   Container(ContainerNode<NodeKind>),
/// }
///
/// impl_node_enum!(NodeKind, Container => ContainerNode<NodeKind>, Image => ImageNode, Text => TextNode);
/// ```
#[macro_export]
macro_rules! impl_node_enum {
  ($name:ident, $($variant:ident => $variant_type:ty),*) => {
    impl $crate::node::Node<$name> for $name {
      fn get_children(&self) -> Option<Vec<&$name>> {
        match self {
          $( $name::$variant(inner) => inner.get_children(), )*
        }
      }

      fn get_style(&self) -> &$crate::node::style::Style {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::get_style(inner), )*
        }
      }

      fn get_style_mut(&mut self) -> &mut $crate::node::style::Style {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::get_style_mut(inner), )*
        }
      }

      fn inherit_style(&mut self, parent: &$crate::node::style::Style) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::inherit_style(inner, parent), )*
        }
      }

      fn before_layout(&mut self) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::before_layout(inner), )*
        }
      }

      fn inherit_style_for_children(&mut self) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::inherit_style_for_children(inner), )*
        }
      }

      fn should_hydrate(&self) -> bool {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::should_hydrate(inner), )*
        }
      }

      fn hydrate(&self, context: &$crate::context::GlobalContext) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::hydrate(inner, context), )*
        }
      }

      fn measure(
        &self,
        context: &$crate::render::RenderContext,
        available_space: $crate::taffy::Size<$crate::taffy::AvailableSpace>,
        known_dimensions: $crate::taffy::Size<Option<f32>>,
      ) -> $crate::taffy::Size<f32> {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::measure(inner, context, available_space, known_dimensions), )*
        }
      }

      fn draw_on_canvas(&self, context: &$crate::render::RenderContext, canvas: &mut $crate::node::draw::FastBlendImage, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::draw_on_canvas(inner, context, canvas, layout), )*
        }
      }

      fn draw_background(&self, context: &$crate::render::RenderContext, canvas: &mut $crate::node::draw::FastBlendImage, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::draw_background(inner, context, canvas, layout), )*
        }
      }

      fn draw_content(&self, context: &$crate::render::RenderContext, canvas: &mut $crate::node::draw::FastBlendImage, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::draw_content(inner, context, canvas, layout), )*
        }
      }

      fn draw_border(&self, context: &$crate::render::RenderContext, canvas: &mut $crate::node::draw::FastBlendImage, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::draw_border(inner, context, canvas, layout), )*
        }
      }

      fn draw_box_shadow(&self, context: &$crate::render::RenderContext, canvas: &mut $crate::node::draw::FastBlendImage, layout: $crate::taffy::Layout) {
        match self {
          $( $name::$variant(inner) => <_ as $crate::node::Node<$name>>::draw_box_shadow(inner, context, canvas, layout), )*
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
