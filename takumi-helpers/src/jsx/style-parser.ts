import type { CSSProperties } from "react";
import type { PartialStyle } from "../types";
import { camelToSnakeCase, removeGlobalValues } from "../utils";

function processCssProp<K extends keyof CSSProperties>(
  style: PartialStyle,
  cssProp: K,
  sourceValue: CSSProperties[K],
) {
  const cleanValue = removeGlobalValues(sourceValue);

  if (cleanValue === undefined || cleanValue === null) return;

  const styleKey = camelToSnakeCase(cssProp);

  try {
    (style as Record<string, unknown>)[styleKey as string] = cleanValue;
  } catch (error) {
    console.warn(`Failed to parse ${cssProp}:`, error);
  }
}

export function parseStyle(source?: CSSProperties): PartialStyle | undefined {
  if (!source) return undefined;

  const result: PartialStyle = {};

  // Process standard properties
  for (const [cssProp, sourceValue] of Object.entries(source)) {
    processCssProp(result, cssProp as keyof CSSProperties, sourceValue);
  }

  return result;
}
