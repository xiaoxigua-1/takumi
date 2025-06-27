import type { AnyNode } from "@takumi-rs/helpers";
import { useMemo, useState } from "react";
import { useTakumi } from "~/hooks/use-takumi";

export function ImageRender({ node }: { node: AnyNode }) {
  const takumi = useTakumi();
  const [error, setError] = useState<string>();

  const url = useMemo(() => {
    if (!takumi) return;

    setError(undefined);

    try {
      const render = new takumi.Renderer();
      return render.renderAsDataUrl(
        node,
        1280,
        720,
        takumi.ImageOutputFormat.WebP,
      );
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  }, [node, takumi]);

  if (!takumi) return <p>Loading WASM...</p>;

  if (error) return <p>Render Error: {error}</p>;

  if (!url) return <p>Rendering...</p>;

  return <img src={url} alt="Rendered" className="w-auto" />;
}
