# @takumi/core

## Usage

```ts
import { Renderer } from "@takumi/core";
import { container, image } from "@takumi/helpers";
import { writeFile } from "fs/promises";

const render = new Renderer({
  fonts: [],
});

const buffer = render.render(
  container({
    children: [
      image("https://yeecord.com/img/logo.png", {
        width: 128,
        height: 128,
      }),
    ],
    background_color: 0xffffff,
    width: {
      percentage: 100,
    },
    height: {
      percentage: 100,
    },
  })
);

await writeFile("./demo.webp", buffer);
```
