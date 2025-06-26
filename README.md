# Takumi

> 🚧 This project is under development.

_Takumi (匠) means "artisan" or "craftsman" in Japanese._

A library for generating images using CSS Flexbox layout. Available for Rust, Node.js, and WebAssembly. 

It works by rendering a tree of nodes, like a simplified browser engine that outputs images instead of rendering to a screen.

Check the [documentation](https://takumi.kane.tw) for more info.

## Packages

### Rust Crates

- **[`takumi`](takumi/)** - Core library for layout and rendering
- **[`takumi-server`](takumi-server/)** - HTTP server for image generation

### JavaScript/TypeScript

- **[`@takumi-rs/core`](takumi-napi-core/)** - N-API bindings for Node.js
- **[`@takumi-rs/wasm`](takumi-wasm/)** - WebAssembly bindings for browsers
- **[`@takumi-rs/helpers`](takumi-helpers/)** - TypeScript utilities

## License

Licensed under the terms in the [LICENSE](LICENSE) file.
