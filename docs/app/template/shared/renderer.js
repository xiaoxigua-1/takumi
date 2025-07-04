import "./styles.css";
import init, { Renderer } from "@takumi-rs/wasm";
import wasmUrl from "@takumi-rs/wasm/takumi_wasm_bg.wasm?url";

await init(wasmUrl);

export const renderer = new Renderer();
