import * as path from "node:path";
import {
  loader,
  type MetaData,
  type PageData,
  type Source,
  type VirtualFile,
} from "fumadocs-core/source";
import matter from "gray-matter";
import defaultAttributes from "lucide-react/dist/esm/defaultAttributes";
import { __iconNode as axeIconNode } from "lucide-react/dist/esm/icons/axe";
import { __iconNode as bookIconNode } from "lucide-react/dist/esm/icons/book";
import { __iconNode as brainIconNode } from "lucide-react/dist/esm/icons/brain";
import { __iconNode as flaskConicalIconNode } from "lucide-react/dist/esm/icons/flask-conical";
import { __iconNode as leafIconNode } from "lucide-react/dist/esm/icons/leaf";
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

const iconProps = {
  ...defaultAttributes,
  color: "currentColor",
  strokeWidth: 2,
  width: 24,
  height: 24,
};

const icons = {
  Leaf: leafIconNode,
  Brain: brainIconNode,
  Book: bookIconNode,
  FlaskConical: flaskConicalIconNode,
  Axe: axeIconNode,
};

export const source = loader({
  icon(icon) {
    if (icon && icon in icons) {
      return createElement(
        "svg",
        iconProps,
        icons[icon as keyof typeof icons].map(([tag, attrs]) =>
          createElement(tag, attrs),
        ),
      );
    }
  },
  source: {
    files: virtualFiles,
  } as Source<{
    pageData: PageData & {
      index?: boolean;
      content: string;
    };
    metaData: MetaData;
  }>,
  baseUrl: "/docs",
});
