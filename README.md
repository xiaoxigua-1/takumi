<img src="./assets/images/takumi.svg" alt="Takumi" width="64" />

# Takumi

_Takumi (匠 in Japanese) means craftsman 🪓_. It's a library for rendering your React components to images.

For newcomers, check out the [Getting Started](https://takumi.kane.tw/docs/getting-started) documentation for installation and usage.

## Why building another satori alternative?

- All in one. No need to output SVG then have [resvg-js](https://github.com/thx/resvg-js) rendering it again to output PNG.
- Minimal binary size targets to run everywhere. Node.js, web, embedded in Rust, pre-built http server.
- Takes your existing JSX components and drops them in. It should just work.
- RTL support.
- WOFF2 font support. Trims your bundle size.
- PNG, JPEG, WebP output support.

## Showcase

Takumi's Open Graph image is generated with Takumi [(source)](./example/twitter-images/components/og-image.tsx).

![Takumi OG Image](./example/twitter-images/output/og-image.png)

## License

Licensed under the terms in the [LICENSE](LICENSE) file.
