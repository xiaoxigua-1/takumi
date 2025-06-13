# takumi-types

Types and helper functions for [takumi](https://github.com/kane50613/takumi).

## Example

```ts
import { container, text, image, style } from "takumi-types";

const rootStyle = style({
  width: 1200,
  height: 630,
});

const root = container(rootStyle, [
  text(
    style({
      font_size: 24,
      font_weight: 700,
      color: [255, 255, 255],
    }),
    "Hello, world!"
  ),
]);

const response = await fetch("https://takumi-server.kane.tw/image", {
  method: "POST",
  body: JSON.stringify({
    root,
  }),
});

const image = await response.arrayBuffer();
```
