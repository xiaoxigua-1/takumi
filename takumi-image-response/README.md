# @takumi-rs/image-response

A universal `ImageResponse` implementation for Takumi in Next.js and other environments.

Checkout the migration guide [From Next.js ImageResponse](https://takumi.kane.tw/docs/migration/migrate-from-image-response) for more details.

## Installation

```bash
npm install @takumi-rs/image-response @takumi-rs/core @takumi-rs/helpers
```

## Usage

```tsx
import ImageResponse from "@takumi-rs/image-response";

export function GET(request: Request) {
  return new ImageResponse(<OgImage />, {
    width: 1200,
    height: 630,
    format: "webp",
    headers: {
      "Cache-Control": "public, immutable, max-age=31536000",
    },
  });
}
```

### Fonts

Takumi comes with full axis [Geist](https://vercel.com/font) and Geist Mono by default.

We have global fonts cache to avoid loading the same fonts multiple times.

If your environment supports top-level await, you can load the fonts in global scope and reuse the fonts array.

```tsx
const fonts = [
  {
    name: "Inter",
    data: await fetch("/fonts/Inter-Regular.ttf").then((res) => res.arrayBuffer()),
    style: "normal",
    weight: 400,
  },
];

new ImageResponse(<OgImage />, { fonts });
```

If your environment doesn't support top-level await, or just want the fonts to get garbage collected after initialization, you can load the fonts like this.

```tsx
let isFontsLoaded = false;

export function GET(request: Request) {
  const fonts = [];

  if (!isFontsLoaded) {
    isFontsLoaded = true;
    fonts = [
      {
        name: "Inter",
        data: await fetch("/fonts/Inter-Regular.ttf").then((res) => res.arrayBuffer()),
        style: "normal",
        weight: 400,
      },
    ];
  }

  return new ImageResponse(<OgImage />, { fonts });
}
```
