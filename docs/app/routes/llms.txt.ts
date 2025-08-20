import { source } from "~/source";

export function loader() {
  const scanned = ["# Takumi Docs"];

  const map = new Map<string, string[]>();
  for (const page of source.getPages()) {
    const dir = page.slugs?.[0] ?? "docs";

    const list = map.get(dir) ?? [];
    const title = page.data.title ?? page.url;
    const description = page.data.description ?? "";

    list.push(`- [${title}](${page.url}): ${description}`);

    map.set(dir, list);
  }

  for (const [key, value] of map) {
    scanned.push(`## ${key}`);
    scanned.push(value.join("\n"));
  }

  return new Response(scanned.join("\n\n"));
}
