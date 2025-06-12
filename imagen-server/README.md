# takumi-server

HTTP server that exposes image generation as a REST API.

### Run from Source

```bash
cargo run --release --bin takumi-server -- --fonts path/to/font.woff2
```

Checkout [Args](src/args.rs) for more options.

### API Endpoint

`POST /image`

Send a JSON payload with your layout definition to generate an image.

The root node should be a [ContainerNode](../takumi/src/node/mod.rs).

## License

Licensed under the terms in the workspace [LICENSE](../LICENSE) file.
