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
  ],
};
