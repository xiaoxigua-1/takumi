import { Renderer } from "@takumi-rs/core";
import { createComponent } from "~/lib/create-renderer";

const renderer = new Renderer();

export async function GET(request: Request) {
  const url = new URL(request.url);
  const name = url.searchParams.get("name") || "Takumi";

  const buffer = await renderer.renderAsync(createComponent(name), {
    width: 1200,
    height: 630,
    format: "WebP",
  });

  return new Response(buffer as BodyInit, {
    headers: {
      "Content-Type": "image/webp",
    },
  });
}
