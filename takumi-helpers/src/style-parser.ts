import type { CSSProperties } from "react";
import { parseColor } from "./color-parsing";
import {
  parseAspectRatio,
  parseBoxShadow,
  parseDisplay,
  parseFontWeight,
  parseGridAutoFlow,
  parseGridLine,
  parseInset,
  parseLengthUnit,
  parsePosition,
  parseSideLengthUnits,
} from "./style-parsing";
import type {
  AlignItems,
  JsxParsedStyle,
  JustifyContent,
  Style,
  TextOverflow,
} from "./types";
import {
  camelToSnakeCase,
  type CamelToSnakeCase,
  type RemoveGlobalsAndPrefixed,
  removeGlobalValues,
  type SnakeToCamelCase,
} from "./utils";

const SKIP_PARSING_SYMBOL = Symbol("skip-parsing");

type PropertyParser<
  K extends keyof CSSProperties,
  SnakeK = CamelToSnakeCase<K>,
> = SnakeK extends keyof Style
  ? NonNullable<
      RemoveGlobalsAndPrefixed<CSSProperties[K]>
    > extends JsxParsedStyle[SnakeK]
    ? typeof SKIP_PARSING_SYMBOL
    : (
        value: NonNullable<RemoveGlobalsAndPrefixed<CSSProperties[K]>>,
      ) => JsxParsedStyle[SnakeK]
  : never;

const PROPERTY_PARSERS: {
  [K in StyleKey]: PropertyParser<K>;
} = {
  display: parseDisplay,
  position: parsePosition,
  width: parseLengthUnit,
  height: parseLengthUnit,
  maxWidth: parseLengthUnit,
  maxHeight: parseLengthUnit,
  minWidth: parseLengthUnit,
  minHeight: parseLengthUnit,
  aspectRatio: parseAspectRatio,
  padding: parseSideLengthUnits,
  margin: parseSideLengthUnits,
  flexDirection: SKIP_PARSING_SYMBOL,
  flexWrap: SKIP_PARSING_SYMBOL,
  justifyContent: (v) => v as JustifyContent,
  alignContent: (v) => v as JustifyContent,
  alignItems: (v) => v as AlignItems,
  justifyItems: (v) => v as AlignItems,
  flexBasis: parseLengthUnit,
  gap: parseLengthUnit,
  flexGrow: (v) => Number(v),
  flexShrink: (v) => Number(v),
  borderWidth: parseSideLengthUnits,
  backgroundColor: parseColor,
  boxShadow: parseBoxShadow,
  objectFit: SKIP_PARSING_SYMBOL,
  gridAutoColumns: (v) => {
    if (typeof v === "number") return [v] as const;

    return v.split(" ").map(parseGridLine);
  },
  gridAutoRows: (v) => v,
  gridAutoFlow: parseGridAutoFlow,
  gridColumn: (v) => v,
  gridRow: (v) => v,
  gridTemplateColumns: (v) => v,
  gridTemplateRows: (v) => v,
  textOverflow: (v) => v as TextOverflow,
  borderColor: parseColor,
  color: parseColor,
  fontSize: parseLengthUnit,
  fontFamily: SKIP_PARSING_SYMBOL,
  lineHeight: parseLengthUnit,
  fontWeight: parseFontWeight,
  lineClamp: (v) => {
    if (v === "none") return;

    return v as number;
  },
  borderRadius: (v) => parseSideLengthUnits(v),
  textAlign(value) {
    if (value === "match-parent") {
      return void console.warn(
        "Unsupported value for text-align found:",
        value,
      );
    }

    return value;
  },
  letterSpacing: parseLengthUnit,
};

type StyleKey = SnakeToCamelCase<keyof Style>;

const SKIPPED_PROPERTIES = new Set(["top", "right", "bottom", "left"]);

function processCssProp<K extends keyof CSSProperties>(
  style: Partial<JsxParsedStyle>,
  cssProp: K,
  sourceValue: CSSProperties[K],
) {
  if (SKIPPED_PROPERTIES.has(cssProp)) {
    return;
  }

  const cleanValue = removeGlobalValues(sourceValue);

  if (cleanValue === undefined || cleanValue === null) return;

  const parser = PROPERTY_PARSERS[cssProp as StyleKey];
  if (!parser) {
    console.warn(`No parser found for CSS property: ${cssProp}`);
    return;
  }

  const styleKey = camelToSnakeCase(cssProp) as keyof Style;

  if (parser === SKIP_PARSING_SYMBOL) {
    // @ts-expect-error: Typescript is not smart enough to figure out which style field referring to
    style[styleKey] = cleanValue;
    return;
  }

  try {
    // @ts-expect-error: Function union can't retrieve the correct type for prop
    const parsedValue = parser(cleanValue);
    if (parsedValue != null && parsedValue !== undefined) {
      // @ts-expect-error: Typescript is not smart enough to figure out which style field referring to
      style[styleKey] = parsedValue;
    }
  } catch (error) {
    console.warn(`Failed to parse ${cssProp}:`, error);
  }
}

export function parseStyle(source?: CSSProperties): JsxParsedStyle | undefined {
  if (!source) return undefined;

  const result: Partial<JsxParsedStyle> = {};

  // Process standard properties
  for (const [cssProp, sourceValue] of Object.entries(source)) {
    processCssProp(result, cssProp as keyof CSSProperties, sourceValue);
  }

  // Handle special case for inset
  const inset = parseInset(source);
  if (inset !== undefined) {
    result.inset = inset;
  }

  return Object.keys(result).length > 0
    ? (result as JsxParsedStyle)
    : undefined;
}
