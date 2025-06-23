//! Layout system for the takumi rendering library.
//!
//! This module contains the node-based layout system including:
//! - Node trait and implementations
//! - Layout measurement functionality
//! - Node type macros
//! - Container, text, and image node types

/// Container node implementation
pub mod container_node;
/// Default node kind enum
pub mod default_node;
/// Image node implementation
pub mod image_node;
/// Macros for implementing node traits
pub mod macros;
/// Layout measurement functionality
pub mod measure;
/// Node trait and implementations
pub mod node;
/// Text node implementation
pub mod text_node;
/// Core Node trait definition
pub mod trait_node;

// Macro is exported at crate root due to #[macro_export]
pub use container_node::*;
pub use default_node::*;
pub use image_node::*;
pub use measure::*;
pub use node::*;
pub use text_node::*;
