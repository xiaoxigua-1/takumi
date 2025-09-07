# Takumi

<!-- cargo-rdme start -->

Takumi is a library with different parts to render your React components to images. This crate contains the core logic for layout, rendering.

Checkout the [Getting Started](https://takumi.kane.tw/docs/getting-started) page if you are looking for Node.js / WASM bindings.

## Walkthrough

Create a [`GlobalContext`](https://docs.rs/takumi/latest/takumi/struct.GlobalContext.html) to store image resources, font caches, the instance should be reused to speed up the rendering.

Then call [`render`](https://docs.rs/takumi/latest/takumi/rendering/render/) with [`Node`](https://docs.rs/takumi/latest/takumi/layout/node/trait.Node.html) and [`Viewport`](https://docs.rs/takumi/latest/takumi/layout/viewport/struct.Viewport.html) to get [`RgbaImage`](image::RgbaImage).

Theres a helper function [`write_image`](https://docs.rs/takumi/latest/takumi/rendering/render/fn.write_image.html) to write the image to a destination implements [`Write`](std::io::Write) and [`Seek`](std::io::Seek).

## Example

```rust
use takumi::{
  layout::{
    node::{ContainerNode, TextNode, NodeKind, Node},
    Viewport,
    style::Style,
  },
  rendering::render,
  GlobalContext,
};

// Create a node tree with `ContainerNode` and `TextNode`
let mut node = NodeKind::Container(ContainerNode {
  children: Some(vec![
    NodeKind::Text(TextNode {
      text: "Hello, world!".to_string(),
      style: Style::default(),
    }),
  ]),
  style: Style::default(),
});

// Create a context for storing resources, font caches.
// You should reuse the context to speed up the rendering.
let context = GlobalContext::default();

// Load fonts, pass an optional [`FontInfoOverride`](parley::FontInfoOverride) to override the font's metadata
context.font_context.load_and_store(include_bytes!("../../assets/fonts/noto-sans/google-sans-code-v11-latin-regular.woff2"), None);

// Create a viewport
let viewport = Viewport::new(1200, 630);

// Render the layout to an `RgbaImage`
let image = render(viewport, &context, node).unwrap();
```

## Credits

- [taffy](https://github.com/DioxusLabs/taffy) for the layout system.
- [image](https://github.com/image-rs/image) for the image processing.
- [cosmic-text](https://github.com/kornelski/cosmic-text) for the text rendering.
- [woff2-patched](https://github.com/zimond/woff2-rs) for the font processing.
- [ts-rs](https://github.com/AlephAlpha/ts-rs) for the type-safe serialization.
- [resvg](https://github.com/linebender/resvg) for SVG parsing and rendering.

<!-- cargo-rdme end -->
