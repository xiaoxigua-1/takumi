import type { Config } from "@react-router/dev/config";
import { source } from "./app/source";

export default {
  prerender({ getStaticPaths }) {
    return [
      ...getStaticPaths(),
      ...source
        .getPages()
        .flatMap((page) => {
          if (page.url.startsWith("/docs")) {
            return [page.url, `/og${page.url}/image.webp`];
          }

          return [page.url];
        })
        .flat(),
    ];
  },
  routeDiscovery: {
    mode: "initial",
  },
} satisfies Config;
