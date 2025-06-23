//! Layout system for the takumi rendering library.
//!
//! This module contains the node-based layout system including:
//! - Node trait and implementations
//! - Layout measurement functionality
//! - Node type macros
//! - Container, text, and image node types

/// Macros for implementing node traits
pub mod macros;
/// Layout measurement functionality
pub mod measure;
/// Node trait and implementations
pub mod node;

// Macro is exported at crate root due to #[macro_export]
pub use measure::*;
pub use node::*;
