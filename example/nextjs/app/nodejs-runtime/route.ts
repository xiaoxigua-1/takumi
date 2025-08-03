import type { OutputFormat } from "@takumi-rs/core";
import { createComponent, createRenderer } from "~/lib/create-renderer";

const renderer = createRenderer();

export const runtime = "nodejs";

export async function GET(request: Request) {
  const url = new URL(request.url);
  const name = url.searchParams.get("name") || "World";

  return new Response(
    await renderer.renderAsync(createComponent(name), {
      width: 1200,
      height: 630,
      format: "WebP" as OutputFormat.WebP, // when `isolatedModules` is enabled, you need to use the enum value directly
    }),
    {
      headers: {
        "Content-Type": "image/webp",
      },
    },
  );
}
