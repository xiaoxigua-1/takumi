import {
  container,
  image,
  percentage,
  rem,
  rgba,
  text,
} from "@takumi-rs/helpers";
import { ImageOutputFormat, initSync, Renderer } from "@takumi-rs/wasm";
import module from "@takumi-rs/wasm/takumi_wasm_bg.wasm";
import medium from "../../../assets/fonts/noto-sans/noto-sans-v39-latin-500.woff2";
import bold from "../../../assets/fonts/noto-sans/noto-sans-v39-latin-700.woff2";
import { fetchLogo } from "./utils";

initSync({ module });

const renderer = new Renderer();

renderer.loadFont(new Uint8Array(bold));
renderer.loadFont(new Uint8Array(medium));

let logo: string;

export default {
  async fetch(request) {
    logo ??= await fetchLogo();

    const { searchParams } = new URL(request.url);

    const name = searchParams.get("name") || "Wizard";

    const webp = renderer.render(
      container({
        width: percentage(100),
        height: percentage(100),
        backgroundColor: 0,
        color: 0xffffff,
        padding: rem(4),
        flexDirection: "column",
        gap: rem(0.5),
        children: [
          text(`Hello, ${name}!`, {
            fontSize: 64,
            fontWeight: 700,
          }),
          text("Nothing beats a Jet2 holiday!", {
            fontSize: 32,
            color: rgba(255, 255, 255, 0.8),
          }),
          image(logo, {
            position: "absolute",
            inset: ["auto", "auto", rem(4), rem(4)],
            width: 96,
            height: 96,
            borderRadius: percentage(50),
          }),
        ],
      }),
      1200,
      630,
      ImageOutputFormat.WebP,
    );

    return new Response(webp, {
      headers: {
        "Content-Type": "image/webp",
        "Cache-Control":
          "private, max-age=0, no-cache, no-store, must-revalidate",
      },
    });
  },
} satisfies ExportedHandler<Env>;
