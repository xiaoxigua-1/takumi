import { test, expect } from "bun:test";
import { Renderer } from "../index";
import { container, image, percentage, rem, text } from "@takumi-rs/helpers";
import { Glob } from "bun";

let renderer = new Renderer();

const logo = "https://yeecord.com/img/logo.png";

test("preloadImageAsync", async () => {
  await renderer.preloadImageAsync(logo);
});

test("loadFontsAsync", async () => {
  const glob = new Glob("../assets/fonts/**/*.{woff2,ttf}");
  const files = await Array.fromAsync(glob.scan());

  const buffers = await Promise.all(
    files.map((file) => Bun.file(file).arrayBuffer())
  );

  const count = await renderer.loadFontsAsync(buffers);
  expect(count).toBe(files.length);
});

test("renderAsync", async () => {
  const result = await renderer.renderAsync(
    container({
      children: [
        image(logo, {
          width: 128,
          height: 128,
          border_radius: percentage(50),
        }),
        text("Hello World"),
      ],
      padding: rem(2),
      flex_direction: "column",
      justify_content: "center",
      align_items: "center",
      gap: rem(1.5),
      background_color: 0xffffff,
      width: percentage(100),
      height: percentage(100),
    }),
    1200,
    630
  );

  expect(result).toBeInstanceOf(Buffer);
});

test("clearImageStore", () => renderer.clearImageStore());
