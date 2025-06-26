import type { Config } from "@react-router/dev/config";
import { source } from "./app/source";

export default {
  prerender({ getStaticPaths }) {
    return [...getStaticPaths(), ...source.getPages().map((page) => page.url)];
  },
  routeDiscovery: {
    mode: "initial",
  },
} satisfies Config;
