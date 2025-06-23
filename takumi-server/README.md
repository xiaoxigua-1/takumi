# takumi-server

HTTP server that exposes image generation as a REST API.

### Run from Source

```bash
cargo run --release --bin takumi-server -- -f path/to/font.woff2
```

Checkout [Args](src/args.rs) for more options.

### API Endpoint

`POST /image`

Send a JSON payload with your layout definition to generate an image.

The root node can be any of the following node types:
- **ContainerNode**: For layout containers with flexbox properties and child nodes
- **TextNode**: For rendering text with styling and typography options  
- **ImageNode**: For displaying images with various fit modes and styling

See the [node implementations](../takumi/src/layout/node.rs) for detailed type definitions and available properties.

### Example Request

```json
{
  "type": "container",
  "width": 1200,
  "height": 630,
  "background_color": [255, 255, 255],
  "padding": 32,
  "justify_content": "center",
  "align_items": "center",
  "children": [
    {
      "type": "text",
      "text": "Hello, World!",
      "font_size": 48,
      "color": 0
    }
  ]
}
```

## License

Licensed under the terms in the workspace [LICENSE](../LICENSE) file.
