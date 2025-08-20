# Takumi

<!-- cargo-rdme start -->

Takumi is a library with different parts to render your React components to images. This crate contains the core logic for layout, rendering.

Checkout the [Getting Started](https://takumi.kane.tw/docs/getting-started) page if you are looking for Node.js / WASM bindings.

## Walkthrough

Everything starts with a `ImageRenderer` instance, it takes [`Node`](https://docs.rs/takumi/latest/takumi/layout/node/trait.Node.html) tree as input then calculate the layout.

You can then draw the layout to an [`RgbaImage`](image::RgbaImage).

## Example

```rust
use takumi::{
  layout::{
    node::{ContainerNode, TextNode, NodeKind, Node},
    Viewport,
    style::Style,
  },
  rendering::ImageRenderer,
  GlobalContext,
};

// Create a node tree with `ContainerNode` and `TextNode`
let mut node = ContainerNode {
  children: Some(vec![
    TextNode {
      text: "Hello, world!".to_string(),
      style: Style::default(),
    }.into(),
  ]),
  style: Style::default(),
};

// Inherit styles for children
node.inherit_style_for_children();

// Create a context for storing resources, font caches.
// You should reuse the context to speed up the rendering.
let context = GlobalContext::default();

// Load fonts
context.font_context.load_and_store(include_bytes!("../../assets/fonts/noto-sans/google-sans-code-v11-latin-regular.woff2").to_vec());

// Create a renderer with a viewport
// You should create a new renderer for each render.
let mut renderer: ImageRenderer<NodeKind> = ImageRenderer::new(Viewport::new(1200, 630));

// Construct the taffy tree, this will calculate the layout and store the result in the renderer.
renderer.construct_taffy_tree(node.into(), &context);

// Draw the layout to an `RgbaImage`
let image = renderer.draw(&context).unwrap();
```

## Credits

- [taffy](https://github.com/DioxusLabs/taffy) for the layout system.
- [image](https://github.com/image-rs/image) for the image processing.
- [cosmic-text](https://github.com/kornelski/cosmic-text) for the text rendering.
- [woff2-patched](https://github.com/zimond/woff2-rs) for the font processing.
- [ts-rs](https://github.com/AlephAlpha/ts-rs) for the type-safe serialization.
- [resvg](https://github.com/linebender/resvg) for SVG parsing and rendering.

<!-- cargo-rdme end -->
