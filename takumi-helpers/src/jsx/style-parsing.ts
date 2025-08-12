import type { Property } from "csstype";
import type { CSSProperties } from "react";
import { em, fr, percentage, rem, vh, vw } from "../helpers";
import type {
  BoxShadow,
  Display,
  FontWeight,
  GridAutoFlow,
  GridLine,
  GridRepeatTrack,
  GridRepetitionCount,
  GridTemplateComponent,
  GridTrackSize,
  ImageScalingAlgorithm,
  LengthUnit,
  Position,
  SidesValue,
  TextOverflow,
} from "../types";
import { parseColor } from "./color-parsing";

export function parseLengthUnit(value: Property.Width | number): LengthUnit {
  if (typeof value === "number") return value;

  if (value === "auto" || value === "min-content" || value === "max-content")
    return value as LengthUnit;

  const match = value.match(/^(-?[\d.]+)(.*)$/);
  if (!match) return "auto";

  const [, num, unit] = match;
  if (!num) return "auto";

  const numValue = Number.parseFloat(num);

  switch (unit) {
    case "%":
      return percentage(numValue);
    case "rem":
      return rem(numValue);
    case "em":
      return em(numValue);
    case "vh":
      return vh(numValue);
    case "vw":
      return vw(numValue);
    case "px":
    case "":
      return numValue;
    default:
      return numValue;
  }
}

export function parseSideLengthUnits(
  value: string | number,
): SidesValue<LengthUnit> {
  if (typeof value === "number") {
    return parseLengthUnit(value);
  }

  const parts = value.trim().split(/\s+/);
  const units = parts.map(parseLengthUnit);

  // Handle CSS shorthand notation
  if (units.length === 1) {
    // All sides same value
    return units[0] as SidesValue<LengthUnit>;
  }
  if (units.length === 2 || units.length === 4) {
    // top/bottom, left/right
    return units as SidesValue<LengthUnit>;
  }
  if (
    units.length === 3 &&
    units[0] !== undefined &&
    units[1] !== undefined &&
    units[2] !== undefined
  ) {
    // top, left/right, bottom - expand to 4 values
    return [units[0], units[1], units[2], units[1]];
  }

  throw new Error(
    `Invalid sides value: ${value}. Expected 1 to at most 4 values.`,
  );
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

export function parseInset(
  source: CSSProperties,
): SidesValue<LengthUnit> | undefined {
  // If no inset or individual position properties, return undefined
  if (
    source.inset === undefined &&
    source.top === undefined &&
    source.right === undefined &&
    source.bottom === undefined &&
    source.left === undefined
  ) {
    return undefined;
  }

  // Start with inset values or default to 0
  let topValue: LengthUnit = 0;
  let rightValue: LengthUnit = 0;
  let bottomValue: LengthUnit = 0;
  let leftValue: LengthUnit = 0;

  if (source.inset !== undefined) {
    const baseValue = source.inset;
    if (baseValue !== undefined) {
      const baseSides = parseBaseSides(baseValue);
      topValue = baseSides.top;
      rightValue = baseSides.right;
      bottomValue = baseSides.bottom;
      leftValue = baseSides.left;
    }
  }

  // Override with individual position properties if they exist
  if (source.top !== undefined) {
    topValue = parseLengthUnit(source.top);
  }
  if (source.right !== undefined) {
    rightValue = parseLengthUnit(source.right);
  }
  if (source.bottom !== undefined) {
    bottomValue = parseLengthUnit(source.bottom);
  }
  if (source.left !== undefined) {
    leftValue = parseLengthUnit(source.left);
  }

  // If all values are the same, return a single value
  if (
    topValue === rightValue &&
    rightValue === bottomValue &&
    bottomValue === leftValue
  ) {
    return topValue;
  }

  return [topValue, rightValue, bottomValue, leftValue];
}

export function parseAspectRatio(aspectRatio: Property.AspectRatio): number {
  if (typeof aspectRatio === "number") return aspectRatio;

  if (typeof aspectRatio === "string" && aspectRatio.includes("/")) {
    const [numerator, denominator] = aspectRatio
      .split("/")
      .map(Number.parseFloat);

    if (numerator === undefined || denominator === undefined) {
      throw new Error(
        `Invalid aspect ratio: ${aspectRatio}. Expected format 'width/height'.`,
      );
    }

    if (denominator === 0) {
      throw new Error(
        `Invalid aspect ratio: ${aspectRatio}. Denominator cannot be zero.`,
      );
    }

    return numerator / denominator;
  }

  if (typeof aspectRatio === "string") {
    return Number.parseFloat(aspectRatio);
  }

  throw new Error(`Invalid aspect ratio: ${aspectRatio}`);
}

export function parseDisplay(display: Property.Display): Display {
  switch (display) {
    case "flex":
    case "grid":
      return display as Display;
    default:
      console.warn(`Invalid display value: ${display}, fallback to 'flex'.`);
      return "flex" as Display;
  }
}

export function parsePosition(position: Property.Position): Position {
  switch (position) {
    case "relative":
    case "absolute":
      return position as Position;
    default:
      console.warn(
        `Invalid position value: ${position}, fallback to 'relative'.`,
      );
      return "relative" as Position;
  }
}

export function parseBoxShadow(source: Property.BoxShadow): BoxShadow {
  const parts = source.trim().split(/\s+/);
  if (parts.length < 2) {
    throw new Error(`Invalid box-shadow: ${source}`);
  }

  let inset = false;
  let colorPart = "black";
  let startIndex = 0;

  if (parts[0] === "inset") {
    inset = true;
    startIndex = 1;
  }

  const colorRegex = /^(#|rgb|rgba|hsl|hsla|[a-zA-Z]+)/;
  const lengthValues: string[] = [];

  for (let i = startIndex; i < parts.length; i++) {
    const part = parts[i] ?? "";
    if (i === parts.length - 1 && colorRegex.test(part)) {
      colorPart = part;
    } else {
      lengthValues.push(part);
    }
  }

  if (lengthValues.length < 2 || lengthValues.length > 4) {
    throw new Error(`Invalid box-shadow format: ${source}`);
  }

  const [
    offsetX = "0px",
    offsetY = "0px",
    blurRadius = "0px",
    spreadRadius = "0px",
  ] = lengthValues;

  try {
    const color = parseColor(colorPart);

    return {
      color,
      offset_x: parseLengthUnit(offsetX) ?? 0,
      offset_y: parseLengthUnit(offsetY) ?? 0,
      blur_radius: parseLengthUnit(blurRadius) ?? 0,
      spread_radius: parseLengthUnit(spreadRadius) ?? 0,
      inset,
    };
  } catch {
    throw new Error(`Invalid box-shadow color: ${colorPart}`);
  }
}

export function parseGridTrackSize(trackSize: string): GridTrackSize[] {
  const parts = trackSize.trim().split(/\s+/);
  return parts.map((part) => {
    if (part.endsWith("fr")) {
      const frValue = Number.parseFloat(part.slice(0, -2));
      if (Number.isNaN(frValue)) return { fr: 0 };
      return { fr: frValue };
    }
    return parseLengthUnit(part) ?? 0;
  });
}

export function parseGridAutoFlow(
  autoFlow: Property.GridAutoFlow,
): GridAutoFlow {
  switch (autoFlow) {
    case "row":
    case "column":
    case "row dense":
    case "column dense":
      return autoFlow.replace(" ", "-") as GridAutoFlow;
    default:
      console.warn(
        `Invalid grid-auto-flow value: ${autoFlow}, fallback to 'row'.`,
      );
      return "row" as GridAutoFlow;
  }
}

export function parseGridLine(gridLine: string): GridLine {
  if (gridLine.startsWith("span ")) {
    const spanValue = Number.parseInt(gridLine.slice(5));
    if (Number.isNaN(spanValue)) {
      throw new Error(`Invalid span value: ${gridLine}`);
    }
    return { start: spanValue, end: null };
  }

  if (gridLine.includes("/")) {
    const parts = gridLine.split("/");
    if (parts.length !== 2) {
      throw new Error(`Invalid grid line format: ${gridLine}`);
    }

    const [startStr = "", endStr = ""] = parts.map((s) => s.trim());
    const start =
      startStr === "auto" ? null : Number.parseInt(startStr) || startStr;
    const end = endStr === "auto" ? null : Number.parseInt(endStr) || endStr;

    return { start, end };
  }

  if (gridLine === "auto") {
    return { start: null, end: null };
  }

  const lineNumber = Number.parseInt(gridLine);
  if (Number.isNaN(lineNumber)) {
    return { start: gridLine, end: null };
  }
  return { start: lineNumber, end: null };
}

export function parseGridTemplateComponent(
  trackFunction: string,
): GridTemplateComponent[] {
  const repeatMatch = trackFunction.match(/repeat\(([^,]+),\s*(.+)\)/);
  if (repeatMatch) {
    const result = parseRepeatFunction(repeatMatch);
    if (!result) {
      throw new Error(`Invalid repeat function: ${trackFunction}`);
    }
    return result;
  }

  return parseSimpleTrackSizes(trackFunction);
}

function parseRepeatFunction(
  match: RegExpMatchArray,
): GridTemplateComponent[] | undefined {
  const [, repetition, trackSizes] = match;
  if (!repetition || !trackSizes) return;

  let repetitionValue: GridRepetitionCount;
  if (repetition === "auto-fill") {
    repetitionValue = "auto-fill";
  } else if (repetition === "auto-fit") {
    repetitionValue = "auto-fit";
  } else {
    const numRepetition = Number.parseInt(repetition);
    if (Number.isNaN(numRepetition)) return;
    repetitionValue = numRepetition;
  }

  // Parse track sizes with potential names
  const sizes = parseGridRepeatTracks(trackSizes);

  return [{ repeat: [repetitionValue, sizes] }];
}

function parseGridRepeatTracks(trackSizes: string): GridRepeatTrack[] {
  const tokens = tokenizeGridTrackList(trackSizes);
  const result: GridRepeatTrack[] = [];
  
  // Keep track of names that come before the next size
  let pendingNames: string[] = [];
  
  for (let i = 0; i < tokens.length; i++) {
    const token = tokens[i];
    if (token.type === "names") {
      // Accumulate names
      pendingNames.push(...token.value);
    } else if (token.type === "size") {
      // Parse the size
      let size: GridTrackSize;
      if (token.value.endsWith("fr")) {
        const frValue = Number.parseFloat(token.value.slice(0, -2));
        size = { fr: frValue };
      } else {
        size = parseLengthUnit(token.value) ?? 0;
      }
      
      // Create a track with the accumulated names
      result.push({ size, names: pendingNames });
      
      // Reset pending names
      pendingNames = [];
    }
  }
  
  return result;
}

function tokenizeGridTrackList(input: string): Array<{ type: "size" | "names"; value: string | string[] }> {
  const tokens: Array<{ type: "size" | "names"; value: string | string[] }> = [];
  let i = 0;
  
  while (i < input.length) {
    // Skip whitespace
    while (i < input.length && /\s/.test(input[i])) {
      i++;
    }
    
    if (i >= input.length) break;
    
    // Check for line names in brackets
    if (input[i] === "[") {
      const bracketEnd = input.indexOf("]", i);
      if (bracketEnd !== -1) {
        const namesStr = input.substring(i + 1, bracketEnd).trim();
        const names = namesStr ? namesStr.split(/\s+/) : [];
        tokens.push({ type: "names", value: names });
        i = bracketEnd + 1;
        continue;
      }
    }
    
    // Parse track size (until next space or bracket)
    let sizeEnd = i;
    while (sizeEnd < input.length && !/\s/.test(input[sizeEnd]) && input[sizeEnd] !== "[") {
      sizeEnd++;
    }
    
    const size = input.substring(i, sizeEnd);
    if (size) {
      tokens.push({ type: "size", value: size });
    }
    
    i = sizeEnd;
  }
  
  return tokens;
}

function parseSimpleTrackSizes(trackFunction: string): GridTemplateComponent[] {
  const tokens = tokenizeGridTrackList(trackFunction);
  const result: GridTemplateComponent[] = [];
  
  // Keep track of names that come before the next size
  let pendingNames: string[] = [];
  
  for (let i = 0; i < tokens.length; i++) {
    const token = tokens[i];
    if (token.type === "names") {
      // Accumulate names
      pendingNames.push(...token.value);
    } else if (token.type === "size") {
      // Parse the size
      let size: GridTrackSize;
      if (token.value.endsWith("fr")) {
        const frValue = Number.parseFloat(token.value.slice(0, -2));
        size = { fr: frValue };
      } else {
        size = parseLengthUnit(token.value) ?? 0;
      }
      
      // If we have names, we need to create a repeat track with one item
      if (pendingNames.length > 0) {
        const repeatTrack: GridRepeatTrack = { size, names: pendingNames };
        result.push({ repeat: [1, [repeatTrack]] });
      } else {
        result.push({ single: size });
      }
      
      // Reset pending names
      pendingNames = [];
    }
  }
  
  return result;
}

export function parseTextOverflow(
  textOverflow: Property.TextOverflow,
): TextOverflow {
  switch (textOverflow) {
    case "ellipsis":
    case "clip":
      return textOverflow as TextOverflow;
    default:
      console.warn(
        `Invalid text-overflow value: ${textOverflow}, fallback to 'clip'.`,
      );
      return "clip" as TextOverflow;
  }
}

export function parseFontWeight(fontWeight: Property.FontWeight): FontWeight {
  if (typeof fontWeight === "number") {
    return Math.max(1, Math.min(1000, fontWeight));
  }

  if (typeof fontWeight === "string") {
    switch (fontWeight) {
      case "normal":
        return 400;
      case "bold":
        return 700;
      case "lighter":
        return 300;
      case "bolder":
        return 600;
      default: {
        const numericWeight = Number.parseInt(fontWeight);
        if (!Number.isNaN(numericWeight)) {
          return Math.max(1, Math.min(1000, numericWeight));
        }
      }
    }
  }

  console.warn(`Invalid font-weight value: ${fontWeight}, fallback to 400.`);
  return 400;
}

export function parseGridAutoRows(
  value: string | number | GridTrackSize[],
): GridTrackSize[] {
  if (typeof value === "string") {
    return parseGridTrackSize(value);
  }
  if (Array.isArray(value)) {
    return value as GridTrackSize[];
  }
  return typeof value === "number"
    ? ([value] as GridTrackSize[])
    : ([fr(1)] as GridTrackSize[]);
}

export function parseGridRow(value: string | number): GridLine {
  if (typeof value === "string") {
    return parseGridLine(value);
  }
  if (typeof value === "number") {
    return { start: value, end: null };
  }
  return value;
}

export function parseGridTemplateColumns(
  value: string | number | GridTemplateComponent[],
): GridTemplateComponent[] {
  if (typeof value === "string") {
    return parseGridTemplateComponent(value);
  }
  if (Array.isArray(value)) {
    return value as GridTemplateComponent[];
  }
  return typeof value === "number"
    ? ([{ single: value }] as GridTemplateComponent[])
    : ([{ single: 0 }] as GridTemplateComponent[]);
}

export function parseGridTemplateRows(
  value: string | number | GridTemplateComponent[],
): GridTemplateComponent[] {
  if (typeof value === "string") {
    return parseGridTemplateComponent(value);
  }
  if (Array.isArray(value)) {
    return value as GridTemplateComponent[];
  }
  return typeof value === "number"
    ? ([{ single: value }] as GridTemplateComponent[])
    : ([{ single: 0 }] as GridTemplateComponent[]);
}

// Add export for tokenizeGridTrackList for testing
export { tokenizeGridTrackList };

export function parseLineClamp(value: string | number): number {
  if (value === "none") return 0;
  return Number(value);
}

export function parseImageRendering(
  value: Property.ImageRendering,
): ImageScalingAlgorithm | undefined {
  switch (value) {
    case "auto":
    case "pixelated":
      return value;
    case "crisp-edges":
      return "pixelated";
    default:
      console.warn(
        `Invalid image-rendering value: ${value}, fallback to 'auto'.`,
      );
      return "auto";
  }
}
