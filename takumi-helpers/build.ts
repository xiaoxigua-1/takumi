import dts from "bun-plugin-dts";

await Bun.build({
  entrypoints: ["./src/index.ts", "./src/jsx/jsx.ts"],
  outdir: "./dist",
  plugins: [dts()],
  minify: true,
  packages: "external"
});
