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
import { Brain, Wrench } from "lucide-react";
import { baseOptions } from "~/layout-config";
import { source } from "~/source";
import type { Route } from "./+types/page";

const components = {
  ...defaultMdxComponents,
  Wrench,
  Brain,
};

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

  const title = `${page.data.title} - Takumi`;

  return (
    <DocsLayout {...baseOptions} links={[]} tree={tree as PageTree.Root}>
      <DocsPage toc={toc}>
        <title>{title}</title>
        <meta name="description" content={page.data.description} />
        <meta name="og:title" content={title} />
        <meta name="og:description" content={page.data.description} />
        <DocsTitle>{page.data.title}</DocsTitle>
        <DocsDescription>{page.data.description}</DocsDescription>
        <DocsBody>
          <Mdx components={components} />
        </DocsBody>
      </DocsPage>
    </DocsLayout>
  );
}
