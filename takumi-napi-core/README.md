# @takumi/core

## Usage

```ts
import { Renderer } from "@takumi/core";
import { container, image } from "@takumi/helpers";
import { writeFile } from "fs/promises";

const render = new Renderer();

const buffer = await render.renderAsync(
  container({
    children: [
      image("https://yeecord.com/img/logo.png", {
        width: 128,
        height: 128,
      }),
    ],
    background_color: 0xffffff,
    width: percentage(100),
    height: percentage(100),
  }),
  1200,
  630
);

await writeFile("./demo.webp", buffer);
```
