import { Renderer, type RenderOptions } from "@takumi-rs/core";
import { fromJsx } from "@takumi-rs/helpers/jsx";
import type { HeadersInit } from "bun";
import type { ReactNode } from "react";

const renderer = new Renderer();

const fontLoadMarker = new WeakSet<Font>();

export type Font = Parameters<typeof renderer.loadFontAsync>[0];

export type ImageResponseOptions = RenderOptions &
  ResponseInit & {
    headers?: HeadersInit;
    signal?: AbortSignal;
    fonts?: Font[];
  };

const defaultOptions: ImageResponseOptions = {
  width: 1200,
  height: 630,
  format: "webp",
};

export function loadFonts(fonts: Font[]) {
  const fontsToLoad = fonts.filter((font) => !fontLoadMarker.has(font));

  for (const font of fontsToLoad) {
    fontLoadMarker.add(font);
  }

  return renderer.loadFontsAsync(fontsToLoad);
}

function createStream(component: ReactNode, options?: ImageResponseOptions) {
  return new ReadableStream({
    async start(controller) {
      try {
        if (options?.fonts) await loadFonts(options.fonts);

        const node = await fromJsx(component);
        const image = await renderer.renderAsync(
          node,
          options ?? defaultOptions,
          options?.signal,
        );

        controller.enqueue(image);
        controller.close();
      } catch (error) {
        controller.error(error);
      }
    },
  });
}

export default class ImageResponse extends Response {
  constructor(component: ReactNode, options?: ImageResponseOptions) {
    const stream = createStream(component, options);
    const headers = new Headers(options?.headers);

    headers.set("Content-Type", "image/webp");

    if (!headers.get("cache-control")) {
      headers.set("cache-control", "public, max-age=0, must-revalidate");
    }

    super(stream, {
      status: options?.status,
      statusText: options?.statusText,
      headers,
    });
  }
}
