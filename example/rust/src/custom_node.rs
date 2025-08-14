use serde::{Deserialize, Serialize};
use takumi::{LengthUnit, Node, Style};

use crate::NodeKind;

/// A specialized node type that renders as a circle.
///
/// The `CircleNode` automatically sets its border radius to 50% to create a perfect circle.
/// It inherits all other styling properties from the base `Style` struct.
///
/// # Example
/// ```rust
/// use takumi::Style;
/// use example::custom_node::CircleNode;
///
/// let circle = CircleNode {
///     style: Style::default(),
/// };
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CircleNode {
  /// The styling properties for the circle node.
  /// This includes dimensions, colors, and other visual properties.
  pub style: Style,
}

impl Node<NodeKind> for CircleNode {
  /// Returns a reference to the node's style properties.
  fn get_style(&self) -> &Style {
    &self.style
  }

  /// Returns a mutable reference to the node's style properties.
  fn get_style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

  /// Modifies the node's style before layout calculation.
  /// Sets the border radius to 50% to ensure the node renders as a perfect circle.
  fn before_layout(&mut self) {
    self.style.inheritable_style.border_radius = Some(LengthUnit::Percentage(50.0).into());
  }
}
