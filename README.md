# takumi

> 🚧 This project is under development.

High-performance Rust library for generating images with CSS Flexbox-like layouts.

_Takumi (匠) means "artisan" or "craftsman" in Japanese - reflecting the precision and artistry required to craft beautiful images through code._

## Crates

### [takumi](takumi/)

Core image rendering library with layout system.

You can use this crate to create your own image rendering system with custom logic or [custom nodes](example/src/custom_node.rs).

### [takumi-server](takumi-server/)

Default implementation of http server for image generation API.

### [@takumi-rs/wasm](takumi-wasm/)

### [@takumi-rs/core](takumi-napi-core/)

N-API binding for takumi

### [@takumi-rs/helpers](takumi-helpers/)

TypeScript bindings for the core library.

## License

Licensed under the terms in the [LICENSE](LICENSE) file.
