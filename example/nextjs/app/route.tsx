import ImageResponse from "@takumi-rs/image-response";
import DocsTemplateV1 from "@takumi-rs/template/docs-template-v1";
import { Axe } from "lucide-react";

export function GET(request: Request) {
  const url = new URL(request.url);
  const name = url.searchParams.get("name") || "Takumi";

  return new ImageResponse(
    <DocsTemplateV1
      title={`Hello from ${name}!`}
      description="Try change the ?name parameter to see the change."
      icon={<Axe color="hsl(354, 90%, 60%)" size={64} />}
      primaryColor="hsla(354, 90%, 54%, 0.3)"
      primaryTextColor="hsl(354, 90%, 60%)"
      site="Takumi"
    />,
    {
      width: 1200,
      height: 630,
      format: "WebP",
    },
  );
}
