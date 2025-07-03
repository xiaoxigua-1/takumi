import "./styles.css";
import { container, percentage } from "@takumi-rs/helpers";
import init, { ImageOutputFormat, Renderer } from "@takumi-rs/wasm";
import wasmUrl from "@takumi-rs/wasm/takumi_wasm_bg.wasm?url";

await init(wasmUrl);

const renderer = new Renderer();

const image = renderer.renderAsDataUrl(
  container({
    background_color: 0xff0000,
    width: percentage(100),
    height: percentage(100),
  }),
  1200,
  630,
  ImageOutputFormat.WebP,
);

const app = document.getElementById("app");

app.innerHTML = `<img src="${image}"/>`;
