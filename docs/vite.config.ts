import path from "node:path";
import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import tsconfigPaths from "vite-tsconfig-paths";

export default defineConfig({
  build: {
    rollupOptions: {
      external: ["shiki"],
      input: path.resolve(__dirname, "index.html"),
    },
  },
  plugins: [tailwindcss(), reactRouter(), tsconfigPaths(), wasm()],
});
