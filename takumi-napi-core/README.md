# @takumi-rs/core

## Usage

```ts
import { Renderer, OutputFormat } from "@takumi-rs/core";
import { container, image } from "@takumi-rs/helpers";
import { writeFile } from "fs/promises";

const render = new Renderer();

const node = container({
  children: [
    image("https://yeecord.com/img/logo.png", {
      width: 128,
      height: 128,
    }),
  ],
  background_color: 0xffffff,
  width: percentage(100),
  height: percentage(100),
});

const buffer = await render.renderAsync(node, {
  width: 1200,
  height: 630,
  format: OutputFormat.WebP
});
```
