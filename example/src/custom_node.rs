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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CircleNode {
  pub style: Style,
}

#[async_trait]
#[typetag::serde(name = "circle")]
impl Node for CircleNode {
  fn get_style(&self) -> &Style {
    &self.style
  }

  fn get_style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

  fn create_taffy_leaf(&self, taffy: &mut TaffyTreeWithNodes) -> Result<NodeId, TaffyError> {
    taffy.new_leaf_with_context(self.style.clone().into(), Box::new(self.clone()))
  }

  fn before_layout(&mut self) {
    self.style.inheritable_style.border_radius = Some(SidesValue::SingleValue(
      ValuePercentageAuto::Percentage(0.5),
    ));
  }
}
