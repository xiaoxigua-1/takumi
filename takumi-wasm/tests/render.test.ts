import { describe, expect, test } from "bun:test";
import { writeFile } from "node:fs/promises";
import { container, image, percentage, rem, text } from "@takumi-rs/helpers";
import { Glob } from "bun";
import init, { ImageOutputFormat, Renderer } from "../pkg/takumi_wasm.js";

await init();

let renderer: Renderer;
const fonts = await loadFonts();

const localImagePath = "../assets/images/yeecord.png";

const localImage = await Bun.file(localImagePath).arrayBuffer();
const dataUri = `data:image/png;base64,${Buffer.from(localImage).toString(
  "base64"
)}`;

const node = container({
  children: [
    image(dataUri, {
      width: 96,
      height: 96,
      border_radius: percentage(25),
    }),
    text("Data URI"),
  ],
  justify_content: "center",
  align_items: "center",
  gap: rem(1.5),
  font_size: rem(1.5),
  background_color: 0xffffff,
  width: percentage(100),
  height: percentage(100),
});

async function loadFonts() {
  const glob = new Glob("../assets/fonts/**/*.{woff2,ttf}");
  const files = await Array.fromAsync(glob.scan());
  const buffers = await Promise.all(
    files.map((file) => Bun.file(file).arrayBuffer())
  );
  return buffers.map((buffer) => new Uint8Array(buffer));
}

describe("setup", () => {
  test("new Renderer", () => {
    renderer = new Renderer();

    expect(renderer).toBeDefined();
  });

  test("loadFont", () => {
    for (const font of fonts) renderer.loadFont(font);
  });
});

describe("render", () => {
  test("webp", async () => {
    const result = renderer.render(node, 1200, 630, ImageOutputFormat.WebP);

    await writeFile("./test.webp", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });

  test("png", async () => {
    const result = renderer.render(node, 1200, 630, ImageOutputFormat.Png);

    await writeFile("./test.png", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });

  test("jpeg 75%", async () => {
    const result = renderer.render(node, 1200, 630, ImageOutputFormat.Jpeg, 75);

    await writeFile("./test-75.jpg", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });

  test("jpeg 100%", async () => {
    const result = renderer.render(
      node,
      1200,
      630,
      ImageOutputFormat.Jpeg,
      100
    );

    await writeFile("./test-100.jpg", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });
});
