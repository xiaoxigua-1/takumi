import { NavigationMenuLink } from "fumadocs-ui/components/ui/navigation-menu";
import { BaseLinkItem } from "fumadocs-ui/layouts/links";
import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";

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
      text: "For LLMs",
      type: "menu",
      items: [
        {
          text: "llms.txt",
          url: "/llms.txt",
          description: "Outline of the documentation",
          external: true,
        },
        {
          text: "llms-full.txt",
          url: "/llms-full.txt",
          description: "Full text of the documentation",
          external: true,
        },
      ],
    },
  ],
};
