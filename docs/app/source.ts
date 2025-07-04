import * as path from "node:path";
import {
  loader,
  type MetaData,
  type PageData,
  type Source,
  type VirtualFile,
} from "fumadocs-core/source";
import matter from "gray-matter";
import { icons } from "lucide-react";
import { createElement } from "react";

const files = Object.entries(
  import.meta.glob<true, "raw">("/content/docs/**/*", {
    eager: true,
    query: "?raw",
    import: "default",
  }),
);

const virtualFiles: VirtualFile[] = files.flatMap(([file, content]) => {
  const ext = path.extname(file);
  const virtualPath = path.relative(
    "content/docs",
    path.join(process.cwd(), file),
  );

  if (ext === ".mdx" || ext === ".md") {
    const parsed = matter(content);

    return {
      type: "page",
      path: virtualPath,
      data: {
        ...parsed.data,
        content: parsed.content,
      },
    };
  }

  if (ext === ".json") {
    return {
      type: "meta",
      path: virtualPath,
      data: JSON.parse(content),
    };
  }

  return [];
});

export const source = loader({
  icon(icon) {
    if (icon && icon in icons)
      return createElement(icons[icon as keyof typeof icons]);
  },
  source: {
    files: virtualFiles,
  } as Source<{
    pageData: PageData & {
      content: string;
    };
    metaData: MetaData;
  }>,
  baseUrl: "/docs",
});
