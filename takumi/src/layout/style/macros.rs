/// Creates a custom style struct with selected fields from the main Style struct.
///
/// This macro generates a new struct that contains only the specified fields from
/// the main `Style` struct, along with derive_builder for convenient construction.
/// All setters accept `Into<T>` values for ergonomic usage.
///
/// # Arguments
/// * `$name` - The name of the generated struct
/// * `$($field:ident: $type:ty),*` - Comma-separated list of field_name: field_type pairs
///
/// # Example
/// ```rust
/// use takumi::define_style;
/// use takumi::layout::style::{CssValue, properties::*};
///
/// // Create a style struct with only display and flex properties
/// define_style!(FlexStyle,
///     display: CssValue<Display>,
///     flex_direction: FlexDirection,
///     justify_content: Option<JustifyContent>,
///     align_items: Option<AlignItems>
/// );
///
/// // Usage with derive_builder
/// let flex_style = FlexStyleBuilder::default()
///     .display(Display::Flex)
///     .flex_direction(FlexDirection::Row)
///     .justify_content(JustifyContent::Center)
///     .align_items(AlignItems::Center)
///     .build()
///     .unwrap();
///
/// // Convert to main Style struct
/// let main_style = flex_style.to_style();
/// ```
#[macro_export]
macro_rules! define_style {
  ($($field:ident: $type:ty = $default_value:expr),*) => {
    /// Main styling structure that contains all layout and visual properties.
    #[derive(Debug, Clone, derive_builder::Builder)]
    #[builder(setter(into), default)]
    pub struct Style {
      $(
        /// Field copied from the main Style struct.
        pub $field: $crate::layout::style::CssValue<$type>,
      )*
    }

    impl Default for Style {
      fn default() -> Self {
        Self {
          $(
            $field: $default_value,
          )*
        }
      }
    }
  };
}
