import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";

export const baseOptions: BaseLayoutProps = {
  githubUrl: "https://github.com/kane50613/takumi",
  nav: {
    title: "Takumi",
  },
  links: [
    {
      text: "Documentation",
      url: "/docs/",
      active: "nested-url",
    },
  ],
};
