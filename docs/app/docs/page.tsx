import * as path from "node:path";
import { createCompiler } from "@fumadocs/mdx-remote";
import { executeMdxSync } from "@fumadocs/mdx-remote/client";
import type { PageTree } from "fumadocs-core/server";
import { DocsLayout } from "fumadocs-ui/layouts/docs";
import defaultMdxComponents from "fumadocs-ui/mdx";
import {
  DocsBody,
  DocsDescription,
  DocsPage,
  DocsTitle,
} from "fumadocs-ui/page";
import { source } from "~/source";
import type { Route } from "./+types/page";

export function meta() {
  return [
    { title: "Takumi" },
    {
      name: "description",
      content:
        "A library for generating images using CSS Flexbox layout. Available for Rust, Node.js, and WebAssembly.",
    },
  ];
}
const compiler = createCompiler({
  development: false,
});

export async function loader({ params }: Route.LoaderArgs) {
  const slugs = params["*"].split("/").filter((v) => v.length > 0);
  const page = source.getPage(slugs);
  if (!page) throw new Error("Not found");

  const compiled = await compiler.compileFile({
    path: path.resolve("content/docs", page.file.path),
    value: page.data.content,
  });

  return {
    page,
    compiled: compiled.toString(),
    tree: source.pageTree,
  };
}

export default function Page(props: Route.ComponentProps) {
  const { page, compiled, tree } = props.loaderData;
  const { default: Mdx, toc } = executeMdxSync(compiled);

  return (
    <DocsLayout
      nav={{
        title: "Takumi",
      }}
      tree={tree as PageTree.Root}
    >
      <DocsPage toc={toc}>
        <DocsTitle>{page.data.title}</DocsTitle>
        <DocsDescription>{page.data.description}</DocsDescription>
        <DocsBody>
          <Mdx components={defaultMdxComponents} />
        </DocsBody>
      </DocsPage>
    </DocsLayout>
  );
}
