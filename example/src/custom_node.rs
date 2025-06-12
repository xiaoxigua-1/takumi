use async_trait::async_trait;
use imagen::{
  node::{
    Node,
    style::{SidesValue, Style, ValuePercentageAuto},
  },
  render::TaffyTreeWithNodes,
  taffy::{NodeId, TaffyError},
};
use serde::{Deserialize, Serialize};

/// A specialized node type that renders as a circle.
///
/// The `CircleNode` automatically sets its border radius to 50% to create a perfect circle.
/// It inherits all other styling properties from the base `Style` struct.
///
/// # Example
/// ```rust
/// use imagen::node::style::Style;
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

#[async_trait]
#[typetag::serde(name = "circle")]
impl Node for CircleNode {
  /// Returns a reference to the node's style properties.
  fn get_style(&self) -> &Style {
    &self.style
  }

  /// Returns a mutable reference to the node's style properties.
  fn get_style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

  /// Creates a new leaf node in the Taffy layout tree.
  fn create_taffy_leaf(&self, taffy: &mut TaffyTreeWithNodes) -> Result<NodeId, TaffyError> {
    taffy.new_leaf_with_context(self.style.clone().into(), Box::new(self.clone()))
  }

  /// Modifies the node's style before layout calculation.
  /// Sets the border radius to 50% to ensure the node renders as a perfect circle.
  fn before_layout(&mut self) {
    self.style.inheritable_style.border_radius = Some(SidesValue::SingleValue(
      ValuePercentageAuto::Percentage(0.5),
    ));
  }
}
