import { HomeLayout } from "fumadocs-ui/layouts/home";
import { ImageEditor } from "~/components/image-editor";
import { baseOptions } from "~/layout-config";

export default function Playground() {
  return (
    <HomeLayout {...baseOptions}>
      <ImageEditor />
    </HomeLayout>
  );
}
