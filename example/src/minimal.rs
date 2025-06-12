// Import necessary modules for file operations and takumi library components
use std::fs::File;

use takumi::{
  context::Context,
  image::ImageFormat,
  node::{
    ContainerNode, TextNode,
    style::{InheritableStyle, Style},
  },
  render::ImageRenderer,
};

/// Generates a "Hello, {name}!" image with specified dimensions and styling
///
/// This function creates a simple image with text using the takumi library.
/// The image is rendered and saved as a WebP file named "output.webp".
pub fn say_hello_to(name: &str) {
  // Create a new context for rendering with default settings
  let context = Context::default();

  // Note: Font loading can be done here using load_woff2_font_to_context
  // Example:
  // load_woff2_font_to_context(&context.font_context, Path::new("fonts/font-name.woff2")).unwrap();

  // Create a text node with custom styling
  // Font size is set to 48.0 and other styles use default values
  let text = TextNode {
    style: Style {
      inheritable_style: InheritableStyle {
        font_size: 48.0.into(),
        ..Default::default()
      },
      ..Default::default()
    },
    text: format!("Hello, {}!", name),
  };

  // Create a root container node that will hold the text
  // Set dimensions to 1200x630 pixels (common size for social media images)
  let root = ContainerNode {
    style: Style {
      width: 1200.0.into(),
      height: 630.0.into(),
      ..Default::default()
    },
    children: vec![Box::new(text)],
  };

  // Create an image renderer from the root node
  let renderer = ImageRenderer::try_from(root).unwrap();

  // Generate the layout tree using Taffy (layout engine)
  let (mut taffy, root_node_id) = renderer.create_taffy_tree();

  // Render the image using the context and layout tree
  let image = renderer.draw(&context, &mut taffy, root_node_id);

  // Create a new file to save the rendered image
  let mut file = File::create("output.webp").unwrap();

  // Write the image to the file in WebP format
  image.write_to(&mut file, ImageFormat::WebP).unwrap();
}
