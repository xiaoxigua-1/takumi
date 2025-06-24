import { expect, test, describe } from "bun:test";
import { writeFile } from "node:fs/promises";
import { container, image, percentage, rem, text } from "@takumi-rs/helpers";
import { Glob } from "bun";
import { OutputFormat, Renderer } from "../index";

const renderer = new Renderer();

const logo = "https://yeecord.com/img/logo.png";
const localImagePath = "../assets/images/yeecord.png";

const localImage = await Bun.file(localImagePath).arrayBuffer();
const dataUri = `data:image/png;base64,${Buffer.from(localImage).toString("base64")}`;

const node = container({
  children: [
    image(logo, {
      width: 96,
      height: 96,
      border_radius: percentage(50),
    }),
    text("Remote"),
    image(localImagePath, {
      width: 96,
      height: 96,
      border_radius: percentage(25),
    }),
    text("Local"),
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
  background_color: 0xffffff,
  width: percentage(100),
  height: percentage(100),
});

describe("setup", () => {
  test("preloadImageAsync", async () => {
    await renderer.preloadImageAsync(logo);
  });

  test("loadFontsAsync", async () => {
    const glob = new Glob("../assets/fonts/**/*.{woff2,ttf}");
    const files = await Array.fromAsync(glob.scan());

    const buffers = await Promise.all(
      files.map((file) => Bun.file(file).arrayBuffer()),
    );

    const count = await renderer.loadFontsAsync(buffers);
    expect(count).toBe(files.length);
  });

  test("loadLocalImageAsync", async () => {
    await renderer.loadLocalImageAsync(localImagePath, localImage);
  });
});

describe("renderAsync", () => {
  const options = {
    width: 1200,
    height: 630,
  };

  test("webp", async () => {
    const result = await renderer.renderAsync(node, {
      ...options,
      format: OutputFormat.WebP
    });

    await writeFile("./test.webp", result);

    expect(result).toBeInstanceOf(Buffer);
  });

  test("png", async () => {
    const result = await renderer.renderAsync(node, {
      ...options,
      format: OutputFormat.Png,
    });

    await writeFile("./test.png", result);

    expect(result).toBeInstanceOf(Buffer);
  });

  test("jpeg", async () => {
    const result = await renderer.renderAsync(node, {
      ...options,
      format: OutputFormat.Jpeg,
    });

    await writeFile("./test.jpg", result);

    expect(result).toBeInstanceOf(Buffer);
  });
})

describe("clean up", () => {
  test("clearImageStore", () => renderer.clearImageStore());
})
