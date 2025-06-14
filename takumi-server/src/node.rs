use serde::Deserialize;
use takumi::{
  impl_node_enum,
  node::{ContainerNode, ImageNode, TextNode},
};

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum NodeKind {
  Image(ImageNode),
  Text(TextNode),
  Container(ContainerNode<NodeKind>),
}

impl_node_enum!(NodeKind, Container => ContainerNode<NodeKind>, Image => ImageNode, Text => TextNode);
