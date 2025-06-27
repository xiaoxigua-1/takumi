import type { Editor as EditorType } from "@monaco-editor/react";
import type { AnyNode } from "@takumi-rs/helpers";
import { useCallback, useEffect, useState } from "react";
import { ImageRender } from "./image-render";

const defaultCode = `import { container, percentage } from "@takumi-rs/helpers";

container({
  background_color: 0xff0000,
  width: percentage(100),
  height: percentage(100),
})`;

export function ImageEditor() {
  const [Editor, setEditor] = useState<typeof EditorType>();
  const [node, setNode] = useState<AnyNode>();
  const [evaluatorUtils, setEvaluatorUtils] = useState<{
    configureMonacoEditor: () => void;
    safeEvaluate: (code: string) => AnyNode | null;
  } | null>(null);

  const handleEditorChange = useCallback(
    (value: string | undefined) => {
      if (!value || !evaluatorUtils) return;
      const newNode = evaluatorUtils.safeEvaluate(value);
      if (newNode) {
        setNode(newNode);
      }
    },
    [evaluatorUtils],
  );

  useEffect(() => {
    // Dynamic import of code evaluator utilities and Monaco Editor
    void import("../../lib/code-evaluator")
      .then((utils) => {
        setEvaluatorUtils(utils);
        utils.configureMonacoEditor();

        return import("@monaco-editor/react");
      })
      .then((module) => {
        setEditor(() => module.default);
        handleEditorChange(defaultCode);
      });
  }, [handleEditorChange]);

  return (
    <div className="grid sm:grid-cols-2 rounded-xl overflow-hidden">
      <div className="bg-[#1e1e1e] border-r flex flex-col aspect-video">
        <div className="bg-[#2d2d30] px-4 py-2 border-b border-gray-600">
          <div className="flex items-center gap-2">
            <span className="ml-2 text-gray-300 text-sm font-medium">
              render.js
            </span>
          </div>
        </div>
        <div className="flex-1">
          {Editor ? (
            <Editor
              height="100%"
              defaultLanguage="typescript"
              defaultValue={defaultCode}
              theme="vs-dark"
              onChange={handleEditorChange}
              options={{
                minimap: { enabled: false },
                scrollBeyondLastLine: false,
                fontSize: 14,
                fontFamily: "Monaco, 'Courier New', monospace",
                lineNumbers: "on",
                renderWhitespace: "selection",
                automaticLayout: true,
              }}
            />
          ) : (
            <div className="flex items-center justify-center h-full text-gray-300">
              Loading editor...
            </div>
          )}
        </div>
      </div>
      <div className="aspect-video grid place-items-center">
        {node && <ImageRender node={node} />}
      </div>
    </div>
  );
}
