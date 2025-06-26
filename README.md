# Takumi

> 🚧 This project is under development.

Library for generating images using a CSS Flexbox-like layout engine. Supports Rust, Node.js (N-API), and WebAssembly runtimes.

_Takumi (匠) means "artisan" or "craftsman" in Japanese - reflecting the precision and artistry required to craft beautiful images through code._

Checkout [documentation](https://takumi.kane.tw) for overview and quick start.

## Overview

Takumi generates images by constructing a tree of nodes, similar to the HTML DOM. You can think of it as a simplified browser engine that outputs an image instead of rendering to a screen.

The core concepts are:

### Nodes

The building blocks of your image. The primary nodes are `ContainerNode` for layout and grouping, `TextNode` for rendering text, `ImageNode` for rendering image.

### Styling

Each node has a `style` property that controls its appearance and layout, using a model inspired by CSS Flexbox.

### Context

A `GlobalContext` holds shared resources like fonts that are used during rendering.

### Rendering

The `ImageRenderer` takes your node tree, calculates the layout, and draws it to an image buffer.

## Packages

This repository is a monorepo containing the following packages:

### Crates

- [`takumi`](takumi/): The core Rust library for layout and rendering.
- [`takumi-server`](takumi-server/): A pre-built HTTP server for image generation.

### NPM Packages

- [`@takumi-rs/core`](takumi-napi-core/): N-API bindings for Node.js environments.
- [`@takumi-rs/wasm`](takumi-wasm/): WebAssembly bindings for browser environments.
- [`@takumi-rs/helpers`](takumi-helpers/): TypeScript helpers for the JS bindings.

## License

Licensed under the terms in the [LICENSE](LICENSE) file.
