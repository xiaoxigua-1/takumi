//! Default node kind enum for the takumi layout system.
//!
//! This module contains the DefaultNodeKind enum which represents
//! a union of all built-in node types in the layout system.

use serde::Deserialize;

use crate::layout::{container_node::ContainerNode, image_node::ImageNode, text_node::TextNode};

/// A union of all node types.
///
/// This enum is used to represent all possible node types in the layout system.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DefaultNodeKind {
  /// A node that displays an image.
  Image(ImageNode),
  /// A node that displays text.
  Text(TextNode),
  /// A node that contains other nodes.
  Container(ContainerNode<DefaultNodeKind>),
}

crate::impl_node_enum!(DefaultNodeKind, Container => ContainerNode<DefaultNodeKind>, Image => ImageNode, Text => TextNode);
