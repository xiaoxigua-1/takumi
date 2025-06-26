import type { Config } from "@react-router/dev/config";
import { source } from "./app/source";

export default {
  ssr: true,
  prerender({ getStaticPaths }) {
    return [...getStaticPaths(), ...source.getPages().map((page) => page.url)];
  },
} satisfies Config;
