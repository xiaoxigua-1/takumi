import { join } from "node:path";
import { type OutputFormat, Renderer } from "@takumi-rs/core";
import { fromJsx } from "@takumi-rs/helpers/jsx";
import { file, Glob, write } from "bun";
import { Component } from "./component";

const renderer = new Renderer({
  persistentImages: [
    {
      src: "takumi.svg",
      data: await file("../../assets/images/takumi.svg").arrayBuffer(),
    },
  ],
});

const glob = new Glob("*.woff2");

for await (const font of glob.scan(
  "../../assets/fonts/plus-jakarta-sans-v11-latin",
)) {
  await renderer.loadFontAsync(
    await file(
      join("../../assets/fonts/plus-jakarta-sans-v11-latin", font),
    ).arrayBuffer(),
  );
}

const component = await fromJsx(<Component />);

const start = performance.now();

const buffer = await renderer.renderAsync(component, {
  width: 1200,
  height: 630,
});

const end = performance.now();

console.log(`Rendered in ${Math.round(end - start)}ms`);

await write("./og-image.png", buffer);
