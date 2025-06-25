import { describe, expect, test } from "bun:test";
import { writeFile } from "node:fs/promises";
import { container, image, percentage, rem, text } from "@takumi-rs/helpers";
import { Glob } from "bun";
import init, {
  ImageOutputFormat,
  Options,
  render,
  Viewport,
} from "../pkg/takumi_wasm.js";

// Initialize WASM module
await init();

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
  test("loadFonts", async () => {
    const fontArrays = await loadFonts();
    expect(fontArrays.length).toBeGreaterThan(0);
  });
});

describe("render", () => {
  test("webp", async () => {
    const viewport = Viewport.new(1200, 630);
    const options = new Options(viewport, ImageOutputFormat.WebP);
    options.fonts = await loadFonts();

    const result = render(node, options);

    await writeFile("./test.webp", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });

  test("webp with debug", async () => {
    const viewport = Viewport.new(1200, 630);
    const options = new Options(viewport, ImageOutputFormat.WebP);
    options.debug = true;
    options.fonts = await loadFonts();

    const result = render(node, options);

    await writeFile("./test-debug.webp", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });

  test("png", async () => {
    const viewport = Viewport.new(1200, 630);
    const options = new Options(viewport, ImageOutputFormat.Png);
    options.fonts = await loadFonts();

    const result = render(node, options);

    await writeFile("./test.png", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });

  test("jpeg 75%", async () => {
    const viewport = Viewport.new(1200, 630);
    const options = new Options(viewport, ImageOutputFormat.Jpeg);
    options.quality = 75;
    options.fonts = await loadFonts();

    const result = render(node, options);

    await writeFile("./test-75.jpg", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });

  test("jpeg 100%", async () => {
    const viewport = Viewport.new(1200, 630);
    const options = new Options(viewport, ImageOutputFormat.Jpeg);
    options.quality = 100;
    options.fonts = await loadFonts();

    const result = render(node, options);

    await writeFile("./test-100.jpg", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });

  test("jpeg with debug and quality", async () => {
    const viewport = Viewport.new(1200, 630);
    const options = new Options(viewport, ImageOutputFormat.Jpeg);
    options.debug = true;
    options.quality = 50;
    options.fonts = await loadFonts();

    const result = render(node, options);

    await writeFile("./test-debug-50.jpg", result);

    expect(result).toBeInstanceOf(Uint8Array);
  });
});
