import {
  FileTabs,
  SandpackLayout,
  SandpackPreview,
  SandpackProvider,
  SandpackStack,
  useActiveCode,
  useSandpack,
} from "@codesandbox/sandpack-react";
import { Editor } from "@monaco-editor/react";
import { useEffect, useState } from "react";
import index from "~/template/index.js?raw";
import renderer from "~/template/shared/renderer.js?raw";
import css from "~/template/shared/styles.css?raw";

async function getPackageVersion(name: string) {
  const response = await fetch(`https://registry.npmjs.org/${name}`, {
    headers: {
      accept: "application/vnd.npm.install-v1+json",
    },
  });

  const json = (await response.json()) as {
    "dist-tags": {
      latest: string;
    };
  };

  return json["dist-tags"].latest;
}

export function ImageEditor() {
  const [version, setVersion] = useState<string>();

  useEffect(() => {
    void getPackageVersion("@takumi-rs/wasm").then(setVersion);
  }, []);

  return (
    <div className="h-[calc(100dvh-3.5rem)] relative">
      {!version && <p>Fetching Latest Takumi version...</p>}
      {version && (
        <SandpackProvider
          style={{ height: "100%" }}
          customSetup={{
            dependencies: {
              "@takumi-rs/helpers": version,
              "@takumi-rs/wasm": version,
            },
          }}
          files={{
            "index.js": index,
            "shared/renderer.js": renderer,
            "shared/styles.css": css,
            ".version": {
              code: version,
              readOnly: true,
            },
          }}
          template="vite"
          theme="dark"
        >
          <SandpackLayout style={{ height: "100%" }}>
            <MonacoEditor />
            <SandpackPreview
              showRefreshButton={false}
              style={{ height: "100%" }}
            />
          </SandpackLayout>
        </SandpackProvider>
      )}
    </div>
  );
}

function MonacoEditor() {
  const { code, updateCode } = useActiveCode();
  const { sandpack } = useSandpack();

  return (
    <SandpackStack className="m-0 !h-full">
      <FileTabs />
      <Editor
        width="100%"
        height="100%"
        language={getLanguageFromPath(sandpack.activeFile)}
        theme="vs-dark"
        options={{
          wordWrap: "on",
          tabSize: 2,
          minimap: {
            enabled: false,
          },
          stickyScroll: {
            enabled: false,
          },
          scrollbar: {
            useShadows: false,
          },
        }}
        key={sandpack.activeFile}
        defaultValue={code}
        onChange={(value) => updateCode(value || "")}
      />
    </SandpackStack>
  );
}

function getLanguageFromPath(path: string) {
  switch (path.slice(path.lastIndexOf("."))) {
    case ".js":
      return "javascript";

    default:
      return "text";
  }
}
