import { index, type RouteConfig, route } from "@react-router/dev/routes";

export default [
  index("routes/home.tsx"),
  route("/llms-full.txt", "routes/llms-full.txt.ts"),
  route("playground", "routes/playground.tsx"),
  route("docs/*", "docs/page.tsx"),
  route("api/search", "docs/search.ts"),
] satisfies RouteConfig;
