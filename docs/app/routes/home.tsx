import { HomeLayout } from "fumadocs-ui/layouts/home";
import { Link } from "react-router";
import { ImageEditor } from "~/components/image-editor";
import { baseOptions } from "~/layout-config";

export function meta() {
  return [
    { title: "Takumi: Craft Beautiful Images with Code" },
    {
      name: "description",
      content:
        "A library for generating images using CSS Flexbox layout. Available for Rust, Node.js, and WebAssembly.",
    },
  ];
}

export default function Home() {
  return (
    <HomeLayout className="text-center" {...baseOptions}>
      <div className="py-12 p-2">
        <h1 className="text-5xl font-bold mb-4">
          Craft Beautiful Images with Code
        </h1>
        <p className="text-lg text-fd-muted-foreground max-w-2xl mx-auto mb-8">
          A library for generating images using a CSS Flexbox-like layout
          engine. Supports Rust, Node.js (N-API), and WebAssembly runtimes.
        </p>
        <Link
          className="text-sm bg-fd-primary text-fd-primary-foreground rounded-full font-medium px-4 py-2.5"
          to="/docs"
        >
          Open Docs
        </Link>
        <div className="py-12 px-4 max-w-7xl mx-auto">
          <ImageEditor />
        </div>
      </div>
    </HomeLayout>
  );
}
