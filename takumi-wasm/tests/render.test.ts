import { describe, expect, test } from "bun:test";
import { readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { container, image, percentage, rem, text } from "@takumi-rs/helpers";
import { Glob } from "bun";
import init, { ImageOutputFormat, Renderer } from "../pkg";

const fontsGlob = new Glob("**/*.{woff2,ttf}");

async function getFonts() {
  const fonts: Buffer[] = [];

  for await (const file of fontsGlob.scan("../assets/fonts")) {
    fonts.push(await readFile(join("../assets/fonts", file)));
  }

  return fonts;
}

const fonts = await getFonts();
let renderer: Renderer;

const localImagePath = "../assets/images/yeecord.png";

const localImage = await readFile(localImagePath);
const dataUri = `data:image/png;base64,${Buffer.from(localImage).toString(
  "base64",
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

describe("setup", () => {
  test("init", async () => {
    await init({
      module_or_path: readFile("./pkg/takumi_wasm_bg.wasm"),
    });
  });

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
      100,
    );

    await writeFile("./test-100.jpg", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });
});

describe("renderAsDataUrl", () => {
  test("default format (png)", () => {
    const result = renderer.renderAsDataUrl(node, 1200, 630);

    expect(result).toMatch(/^data:image\/png;base64,/);
    expect(result.length).toBeGreaterThan(100);
  });

  test("webp format", () => {
    const result = renderer.renderAsDataUrl(
      node,
      1200,
      630,
      ImageOutputFormat.WebP,
    );

    expect(result).toMatch(/^data:image\/webp;base64,/);
    expect(result.length).toBeGreaterThan(100);
  });

  test("jpeg format with quality", () => {
    const result = renderer.renderAsDataUrl(
      node,
      1200,
      630,
      ImageOutputFormat.Jpeg,
      75,
    );

    expect(result).toMatch(/^data:image\/jpeg;base64,/);
    expect(result.length).toBeGreaterThan(100);
  });

  test("png format explicit", () => {
    const result = renderer.renderAsDataUrl(
      node,
      1200,
      630,
      ImageOutputFormat.Png,
    );

    expect(result).toMatch(/^data:image\/png;base64,/);
    expect(result.length).toBeGreaterThan(100);
  });
});
