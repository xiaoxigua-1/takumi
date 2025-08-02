import type { CSSProperties } from "react";
import type {
  AlignItems,
  JsxParsedStyle,
  JustifyContent,
  LengthUnit,
  SidesValue,
  Style,
  TextOverflow,
} from "../types";
import {
  type CamelToSnakeCase,
  camelToSnakeCase,
  type RemoveGlobalsAndPrefixed,
  removeGlobalValues,
  type SnakeToCamelCase,
} from "../utils";
import { parseColor } from "./color-parsing";
import {
  parseAspectRatio,
  parseBoxShadow,
  parseDisplay,
  parseFontWeight,
  parseGridAutoFlow,
  parseGridAutoRows,
  parseGridRow,
  parseGridTemplateColumns,
  parseGridTemplateRows,
  parseImageRendering,
  parseLengthUnit,
  parseLineClamp,
  parsePosition,
  parseSideLengthUnits,
} from "./style-parsing";

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
  justifySelf: (v) => v as AlignItems,
  alignItems: (v) => v as AlignItems,
  alignSelf: (v) => v as AlignItems,
  justifyItems: (v) => v as AlignItems,
  flexBasis: parseLengthUnit,
  gap: parseLengthUnit,
  flexGrow: (v) => Number(v),
  flexShrink: (v) => Number(v),
  borderWidth: parseSideLengthUnits,
  backgroundColor: parseColor,
  boxShadow: parseBoxShadow,
  objectFit: SKIP_PARSING_SYMBOL,
  imageRendering: parseImageRendering,
  gridAutoColumns: parseGridAutoRows,
  gridAutoRows: parseGridAutoRows,
  gridAutoFlow: parseGridAutoFlow,
  gridColumn: parseGridRow,
  gridRow: parseGridRow,
  gridTemplateColumns: parseGridTemplateColumns,
  gridTemplateRows: parseGridTemplateRows,
  textOverflow: (v) => v as TextOverflow,
  borderColor: parseColor,
  color: parseColor,
  fontSize: parseLengthUnit,
  fontFamily: SKIP_PARSING_SYMBOL,
  lineHeight: parseLengthUnit,
  fontWeight: parseFontWeight,
  lineClamp: parseLineClamp,
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
  inset: (v) => parseSideLengthUnits(v),
};

type StyleKey = SnakeToCamelCase<keyof Style>;

const SKIPPED_PROPERTIES = new Set([
  "top",
  "right",
  "bottom",
  "left",
  "marginTop",
  "marginRight",
  "marginBottom",
  "marginLeft",
  "paddingTop",
  "paddingRight",
  "paddingBottom",
  "paddingLeft",
]);

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
    (style as Record<string, unknown>)[styleKey as string] = cleanValue;
    return;
  }

  try {
    const parsedValue = (parser as (value: unknown) => unknown)(cleanValue);
    if (parsedValue != null && parsedValue !== undefined) {
      (style as Record<string, unknown>)[styleKey as string] = parsedValue;
    }
  } catch (error) {
    console.warn(`Failed to parse ${cssProp}:`, error);
  }
}

function parseBaseSides(baseValue: string | number): {
  top: LengthUnit;
  right: LengthUnit;
  bottom: LengthUnit;
  left: LengthUnit;
} {
  const baseValues = parseSideLengthUnits(baseValue);
  if (typeof baseValues === "number") {
    // Single value for all sides
    return {
      top: baseValues,
      right: baseValues,
      bottom: baseValues,
      left: baseValues,
    };
  }

  if (Array.isArray(baseValues)) {
    if (baseValues.length === 2) {
      // top/bottom, left/right
      return {
        top: baseValues[0],
        right: baseValues[1],
        bottom: baseValues[0],
        left: baseValues[1],
      };
    }
    if (baseValues.length === 4) {
      // top, right, bottom, left (including expanded 3-value form)
      return {
        top: baseValues[0],
        right: baseValues[1],
        bottom: baseValues[2],
        left: baseValues[3],
      };
    }
  }
  // Default to 0 for all sides
  return { top: 0, right: 0, bottom: 0, left: 0 };
}

function shouldReturnBaseValueDirectly(
  source: CSSProperties,
  baseProp: string,
  topProp: string,
  rightProp: string,
  bottomProp: string,
  leftProp: string,
): boolean {
  // Check if we have individual overrides
  const hasIndividualOverrides =
    source[topProp as keyof CSSProperties] !== undefined ||
    source[rightProp as keyof CSSProperties] !== undefined ||
    source[bottomProp as keyof CSSProperties] !== undefined ||
    source[leftProp as keyof CSSProperties] !== undefined;

  // If we only have a base property and no individual overrides, return it directly
  return (
    source[baseProp as keyof CSSProperties] !== undefined &&
    !hasIndividualOverrides
  );
}

function getBaseSides(
  source: CSSProperties,
  baseProp: string,
): {
  top: LengthUnit;
  right: LengthUnit;
  bottom: LengthUnit;
  left: LengthUnit;
} {
  const baseValue = source[baseProp as keyof CSSProperties];
  if (baseValue !== undefined && typeof baseValue !== "object") {
    const baseSides = parseBaseSides(baseValue);
    return baseSides;
  }
  // Default to 0 for all sides
  return { top: 0, right: 0, bottom: 0, left: 0 };
}

function getIndividualSideValue(
  source: CSSProperties,
  sideProp: string,
): LengthUnit {
  const sideValue = source[sideProp as keyof CSSProperties];
  if (sideValue !== undefined && typeof sideValue !== "object") {
    return parseLengthUnit(sideValue);
  }
  return 0;
}

function parseSides(
  source: CSSProperties,
  baseProp: "inset" | "padding" | "margin",
  topProp: "top" | "paddingTop" | "marginTop",
  rightProp: "right" | "paddingRight" | "marginRight",
  bottomProp: "bottom" | "paddingBottom" | "marginBottom",
  leftProp: "left" | "paddingLeft" | "marginLeft",
): SidesValue<LengthUnit> | undefined {
  // If no base property or individual side properties, return undefined
  if (
    source[baseProp] === undefined &&
    source[topProp] === undefined &&
    source[rightProp] === undefined &&
    source[bottomProp] === undefined &&
    source[leftProp] === undefined
  ) {
    return undefined;
  }

  // If we only have a base property and no individual overrides, return it directly
  if (
    shouldReturnBaseValueDirectly(
      source,
      baseProp,
      topProp,
      rightProp,
      bottomProp,
      leftProp,
    )
  ) {
    const baseValue = source[baseProp];
    if (baseValue !== undefined) {
      return parseSideLengthUnits(baseValue);
    }
  }

  // Start with base property values or default to 0
  const baseSides = getBaseSides(source, baseProp);
  let topValue = baseSides.top;
  let rightValue = baseSides.right;
  let bottomValue = baseSides.bottom;
  let leftValue = baseSides.left;

  // Override with individual side properties if they exist
  if (source[topProp] !== undefined) {
    topValue = getIndividualSideValue(source, topProp);
  }
  if (source[rightProp] !== undefined) {
    rightValue = getIndividualSideValue(source, rightProp);
  }
  if (source[bottomProp] !== undefined) {
    bottomValue = getIndividualSideValue(source, bottomProp);
  }
  if (source[leftProp] !== undefined) {
    leftValue = getIndividualSideValue(source, leftProp);
  }

  // If all values are the same, return a single value
  if (
    topValue === rightValue &&
    rightValue === bottomValue &&
    bottomValue === leftValue
  ) {
    return topValue;
  }

  // Return as array
  return [topValue, rightValue, bottomValue, leftValue];
}

function parsePadding(
  source: CSSProperties,
): SidesValue<LengthUnit> | undefined {
  return parseSides(
    source,
    "padding",
    "paddingTop",
    "paddingRight",
    "paddingBottom",
    "paddingLeft",
  );
}

function parseMargin(
  source: CSSProperties,
): SidesValue<LengthUnit> | undefined {
  return parseSides(
    source,
    "margin",
    "marginTop",
    "marginRight",
    "marginBottom",
    "marginLeft",
  );
}

function parseInset(source: CSSProperties): SidesValue<LengthUnit> | undefined {
  return parseSides(source, "inset", "top", "right", "bottom", "left");
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

  // Handle special case for padding
  const padding = parsePadding(source);
  if (padding !== undefined) {
    result.padding = padding;
  }

  // Handle special case for margin
  const margin = parseMargin(source);
  if (margin !== undefined) {
    result.margin = margin;
  }

  return Object.keys(result).length > 0
    ? (result as JsxParsedStyle)
    : undefined;
}
