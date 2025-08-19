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

await renderer.loadFontAsync(
  await file(
    "../../assets/fonts/noto-sans/google-sans-code-v11-latin-regular.woff2",
  ).arrayBuffer(),
);

const component = await fromJsx(<Component />);

const buffer = await renderer.renderAsync(component, {
  width: 1200,
  height: 630,
  format: "WebP" as OutputFormat.WebP,
});

await write("./og-image.webp", buffer);
