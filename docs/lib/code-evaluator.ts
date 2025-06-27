import { Editor, loader } from "@monaco-editor/react";
import type { AnyNode } from "@takumi-rs/helpers";
import * as helpers from "@takumi-rs/helpers";
import typeDefs from "../node_modules/@takumi-rs/helpers/dist/index.d.ts?raw";

// Available modules for import
const availableModules = {
  "@takumi-rs/helpers": helpers,
} as const;

type ModuleName = keyof typeof availableModules;

function parseNamedImports(
  names: string,
  module: typeof helpers
): Record<string, unknown> {
  const imports: Record<string, unknown> = {};
  const importNames = names.split(",").map((name) => name.trim());

  for (const name of importNames) {
    if (!(name in module)) {
      throw new Error(`"${name}" is not exported`);
    }
    imports[name] = (module as Record<string, unknown>)[name];
  }

  return imports;
}

function parseImports(code: string): Record<string, unknown> {
  const importRegex =
    /import\s+(?:\{\s*([^}]+)\s*\}|\*\s+as\s+(\w+)|(\w+))\s+from\s+["']([^"']+)["'];?/g;
  const imports: Record<string, unknown> = {};

  let match = importRegex.exec(code);
  while (match !== null) {
    const [, namedImports, namespaceImport, defaultImport, moduleName] = match;

    if (!(moduleName in availableModules)) {
      throw new Error(`Module "${moduleName}" is not available`);
    }

    const module = availableModules[moduleName as ModuleName];

    if (namedImports) {
      Object.assign(imports, parseNamedImports(namedImports, module));
    } else if (namespaceImport) {
      imports[namespaceImport] = module;
    } else if (defaultImport) {
      imports[defaultImport] = module;
    }

    match = importRegex.exec(code);
  }

  return imports;
}

// Configure Monaco Editor with file system
function configureMonacoEditor() {
  loader.init().then((monaco) => {
    // Create virtual file system models
    const helpersUri = monaco.Uri.parse(
      "file:///node_modules/@takumi-rs/helpers/index.d.ts"
    );

    // Create @takumi-rs/helpers type definitions
    const helpersTypeDef = `declare module "@takumi-rs/helpers" {\n${typeDefs}\n}`;

    monaco.editor.createModel(helpersTypeDef, "typescript", helpersUri);

    // Configure TypeScript compiler options
    monaco.languages.typescript.javascriptDefaults.setCompilerOptions({
      target: monaco.languages.typescript.ScriptTarget.ESNext,
      allowNonTsExtensions: true,
      moduleResolution: monaco.languages.typescript.ModuleResolutionKind.NodeJs,
      module: monaco.languages.typescript.ModuleKind.ESNext,
      noEmit: true,
      esModuleInterop: true,
      jsx: monaco.languages.typescript.JsxEmit.React,
      reactNamespace: "React",
      allowJs: true,
      baseUrl: "file:///",
      paths: {
        "@takumi-rs/helpers": ["node_modules/@takumi-rs/helpers/index"],
      },
      includePackageJsonAutoImports: "auto",
    });

    // Register auto-import completion provider
    monaco.languages.registerCompletionItemProvider("typescript", {
      provideCompletionItems: (model, position) => {
        const word = model.getWordUntilPosition(position);
        const range = {
          startLineNumber: position.lineNumber,
          endLineNumber: position.lineNumber,
          startColumn: word.startColumn,
          endColumn: word.endColumn,
        };

        // Get available exports from helpers
        const exports = Object.keys(availableModules["@takumi-rs/helpers"]);
        const suggestions = exports.map((exportName) => ({
          label: exportName,
          kind: monaco.languages.CompletionItemKind.Function,
          insertText: exportName,
          range,
          detail: `(auto-import) ${exportName}`,
          documentation: `Import ${exportName} from @takumi-rs/helpers`,
          additionalTextEdits: getImportEdits(model.getValue(), exportName),
        }));

        return { suggestions };
      },
    });

    // Enable auto imports
    monaco.languages.typescript.javascriptDefaults.setDiagnosticsOptions({
      noSemanticValidation: false,
      noSyntaxValidation: false,
      diagnosticCodesToIgnore: [2792, 2307],
    });
  });
}

// Helper function to generate import edits that properly replace existing imports
function getImportEdits(currentCode: string, exportName: string): any[] {
  const lines = currentCode.split("\n");
  const importRegex =
    /import\s*\{\s*([^}]+)\s*\}\s*from\s*["']@takumi-rs\/helpers["'];?/;

  // Find existing import line
  let importLineIndex = -1;
  let existingImportMatch = null;

  for (let i = 0; i < lines.length; i++) {
    const match = lines[i].match(importRegex);
    if (match) {
      importLineIndex = i;
      existingImportMatch = match;
      break;
    }
  }

  if (existingImportMatch && importLineIndex >= 0) {
    // Update existing import
    const existingImports = existingImportMatch[1]
      .split(",")
      .map((imp) => imp.trim())
      .filter((imp) => imp.length > 0);

    if (!existingImports.includes(exportName)) {
      existingImports.push(exportName);
      existingImports.sort(); // Keep imports sorted

      const newImportLine = `import { ${existingImports.join(
        ", "
      )} } from "@takumi-rs/helpers";`;

      return [
        {
          range: {
            startLineNumber: importLineIndex + 1,
            startColumn: 1,
            endLineNumber: importLineIndex + 1,
            endColumn: lines[importLineIndex].length + 1,
          },
          text: newImportLine,
        },
      ];
    }
    return []; // Already imported
  }

  // Add new import at the top
  const newImportLine = `import { ${exportName} } from "@takumi-rs/helpers";\n`;

  return [
    {
      range: {
        startLineNumber: 1,
        startColumn: 1,
        endLineNumber: 1,
        endColumn: 1,
      },
      text: newImportLine,
    },
  ];
}

function safeEvaluate(code: string): AnyNode | null {
  try {
    // Extract imports and get the main code
    const imports = parseImports(code);
    const codeWithoutImports = code.replace(/import\s+[^;]+;?\s*/g, "").trim();

    // Create a function with limited scope including imports
    const func = new Function(
      ...Object.keys(imports),
      `"use strict"; return (${codeWithoutImports})`
    );

    return func(...Object.values(imports));
  } catch (error) {
    console.error("Error evaluating code:", error);
    return null;
  }
}

function createEditorModel(defaultCode: string) {
  return loader.init().then((monaco) => {
    const mainUri = monaco.Uri.parse("file:///main.ts");
    return monaco.editor.createModel(defaultCode, "typescript", mainUri);
  });
}

export { configureMonacoEditor, safeEvaluate, Editor, createEditorModel };
