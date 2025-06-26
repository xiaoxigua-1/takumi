import type { Config } from "@react-router/dev/config";
import { source } from "~/source";

export default {
  future: {
    unstable_viteEnvironmentApi: true,
  },
  prerender({ getStaticPaths }) {
    return [...getStaticPaths(), ...source.getPages().map((page) => page.url)];
  },
} satisfies Config;
