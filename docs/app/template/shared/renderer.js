import "./styles.css";
import init, { Renderer } from "@takumi-rs/wasm";
import wasmUrl from "@takumi-rs/wasm/takumi_wasm_bg.wasm?url";

await init(wasmUrl);

export const renderer = new Renderer();

renderer.loadFont(await getFont("noto-sans-v39-latin-500.woff2"));
renderer.loadFont(await getFont("noto-sans-v39-latin-700.woff2"));

async function getFont(file) {
  const response = await fetch(`https://takumi.kane.tw/fonts/${file}`);

  if (!response.ok) {
    throw new Error(`Failed to load font: ${file}`);
  }

  return response.arrayBuffer();
}
