import { describe, expect, test } from "bun:test";
import { writeFile } from "node:fs/promises";
import { container, image, percentage, rem, text } from "@takumi-rs/helpers";
import { Glob } from "bun";
import { OutputFormat, Renderer } from "../index";

const renderer = new Renderer();

const remoteUrl = "https://yeecord.com/img/logo.png";
const localImagePath = "../assets/images/yeecord.png";

const remoteImage = await fetch(remoteUrl).then((r) => r.arrayBuffer());

const localImage = await Bun.file(localImagePath).arrayBuffer();
const dataUri = `data:image/png;base64,${Buffer.from(localImage).toString(
  "base64",
)}`;

const node = container({
  children: [
    image(remoteUrl, {
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

test("Renderer initialization with fonts and images", async () => {
  const font = await Bun.file(
    "../assets/fonts/noto-sans/NotoSansTC-Bold.woff",
  ).arrayBuffer();

  new Renderer({
    fonts: [font],
    persistentImages: [
      {
        src: remoteUrl,
        data: remoteImage,
      },
      {
        src: localImagePath,
        data: localImage,
      },
    ],
    debug: true,
  });
});

describe("setup", () => {
  test("loadFontsAsync", async () => {
    const glob = new Glob("../assets/fonts/**/*.{woff2,ttf}");
    const files = await Array.fromAsync(glob.scan());

    const buffers = await Promise.all(
      files.map((file) => Bun.file(file).arrayBuffer()),
    );

    const count = await renderer.loadFontsAsync(buffers);
    expect(count).toBe(files.length);
  });

  test("putPersistentImageAsync / local", async () => {
    await renderer.putPersistentImageAsync(localImagePath, localImage);
  });

  test("putPersistentImageAsync / remote", async () => {
    await renderer.putPersistentImageAsync(remoteUrl, remoteImage);
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
      format: OutputFormat.WebP,
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

  test("jpeg 75%", async () => {
    const result = await renderer.renderAsync(node, {
      ...options,
      format: OutputFormat.Jpeg,
      quality: 75,
    });

    await writeFile("./test-75.jpg", result);

    expect(result).toBeInstanceOf(Buffer);
  });

  test("jpeg 100%", async () => {
    const result = await renderer.renderAsync(node, {
      ...options,
      format: OutputFormat.Jpeg,
      quality: 100,
    });

    await writeFile("./test-100.jpg", result);

    expect(result).toBeInstanceOf(Buffer);
  });
});

describe("clean up", () => {
  test("clearImageStore", () => renderer.clearImageStore());
});
