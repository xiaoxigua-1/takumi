//! Container node implementation for the takumi layout system.
//!
//! This module contains the ContainerNode struct which is used to group
//! other nodes and apply layout properties like flexbox layout.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::layout::{node::Node, style::Style};

/// A container node that can hold child nodes.
///
/// Container nodes are used to group other nodes and apply layout
/// properties like flexbox layout to arrange their children.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContainerNode<Nodes: Node<Nodes>> {
  /// The styling properties for this container
  #[serde(default)]
  pub style: Style,
  /// The child nodes contained within this container
  pub children: Option<Vec<Nodes>>,
}

impl<Nodes: Node<Nodes>> Node<Nodes> for ContainerNode<Nodes> {
  fn take_children(&mut self) -> Option<Vec<Nodes>> {
    self.children.take()
  }

  fn get_style(&self) -> &Style {
    &self.style
  }

  fn get_style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

  fn inherit_style_for_children(&mut self) {
    let style = self.get_style().clone();

    let Some(children) = &mut self.children else {
      return;
    };

    for child in children.iter_mut() {
      child.inherit_style(&style);
    }
  }
}
