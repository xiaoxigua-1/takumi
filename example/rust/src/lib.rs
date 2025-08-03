use takumi::{ContainerNode, ImageNode, TextNode, impl_node_enum};

use crate::custom_node::CircleNode;

pub mod custom_node;
pub mod minimal;

#[derive(Debug, Clone)]
pub enum NodeKind {
  Container(ContainerNode<NodeKind>),
  Text(TextNode),
  Image(ImageNode),
  Circle(CircleNode),
}

impl_node_enum!(NodeKind, Container => ContainerNode<NodeKind>, Text => TextNode, Image => ImageNode, Circle => CircleNode);
