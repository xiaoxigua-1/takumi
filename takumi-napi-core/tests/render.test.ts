import { test, expect, beforeAll } from "bun:test";
import { Renderer } from "../index";
import { container, image, percentage, rem, text } from "@takumi/helpers";

let renderer = new Renderer();

const logo = "https://yeecord.com/img/logo.png";

test("preloadImageAsync", async () => {
  await renderer.preloadImageAsync(logo);
});

test("loadFont", async () => {
  renderer.loadFont(
    await Bun.file(
      "../assets/noto-sans-tc-v36-chinese-traditional_latin-700.woff2"
    ).arrayBuffer()
  );
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
