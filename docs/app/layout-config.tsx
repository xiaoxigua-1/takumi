import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";

export const llmLinks = [
  {
    text: "llms.txt",
    description: "Outline of the documentation",
    url: "/llms.txt",
  },
  {
    text: "llms-full.txt",
    description: "Full text of the documentation",
    url: "/llms-full.txt",
  },
];

export const baseOptions: BaseLayoutProps = {
  githubUrl: "https://github.com/kane50613/takumi",
  nav: {
    title: (
      <>
        <img src="/logo.svg" alt="Takumi" width={24} height={24} />
        <span className="font-semibold">Takumi</span>
      </>
    ),
  },
  links: [
    {
      text: "Documentation",
      url: "/docs",
      active: "nested-url",
    },
    {
      text: "Playground",
      url: "/playground",
    },
    {
      text: "LLMs",
      type: "menu",
      items: llmLinks,
    },
  ],
};
