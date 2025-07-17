import type { Property } from "csstype";
import type { CSSProperties } from "react";
import { em, fr, percentage, rem, vh, vw } from "../helpers";
import type {
  BoxShadow,
  Display,
  FontWeight,
  GridAutoFlow,
  GridLine,
  GridTrackRepetition,
  GridTrackSize,
  LengthUnit,
  Position,
  SidesValue,
  TextOverflow,
  TrackSizingFunction,
} from "../types";
import { parseColor } from "./color-parsing";

export function parseLengthUnit(value: Property.Width | number): LengthUnit {
  if (typeof value === "number") return value;

  if (value === "auto" || value === "min-content" || value === "max-content")
    return value as LengthUnit;

  const match = value.match(/^(-?[\d.]+)(.*)$/);
  if (!match) return 0;

  const [, num, unit] = match;
  if (!num) return 0;
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
    // top, left/right, bottom
    return [units[0], units[1], units[2], units[1]];
  }

  throw new Error(
    `Invalid sides value: ${value}. Expected 1 to at most 4 values.`,
  );
}

export function parseInset(
  source: CSSProperties,
): SidesValue<LengthUnit> | undefined {
  if (source.inset !== undefined) return parseSideLengthUnits(source.inset);

  const top =
    source.top !== undefined ? parseLengthUnit(source.top) : undefined;
  const right =
    source.right !== undefined ? parseLengthUnit(source.right) : undefined;
  const bottom =
    source.bottom !== undefined ? parseLengthUnit(source.bottom) : undefined;
  const left =
    source.left !== undefined ? parseLengthUnit(source.left) : undefined;

  if (
    top === undefined &&
    right === undefined &&
    bottom === undefined &&
    left === undefined
  ) {
    return;
  }

  return [top ?? 0, right ?? 0, bottom ?? 0, left ?? 0] as const;
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
    case "block":
    case "flex":
    case "grid":
    case "none":
      return display as Display;
    default:
      console.warn(`Invalid display value: ${display}, fallback to 'block'.`);
      return "block" as Display;
  }
}

export function parsePosition(position: Property.Position): Position {
  switch (position) {
    case "relative":
    case "absolute":
      return position as Position;
    case "static":
      return "relative" as Position;
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

export function parseTrackSizingFunction(
  trackFunction: string,
): TrackSizingFunction[] {
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
): TrackSizingFunction[] | undefined {
  const [, repetition, trackSizes] = match;
  if (!repetition || !trackSizes) return;

  let repetitionValue: GridTrackRepetition;
  if (repetition === "auto-fill") {
    repetitionValue = "auto-fill";
  } else if (repetition === "auto-fit") {
    repetitionValue = "auto-fit";
  } else {
    const numRepetition = Number.parseInt(repetition);
    if (Number.isNaN(numRepetition)) return;
    repetitionValue = numRepetition;
  }

  const sizes = trackSizes
    .trim()
    .split(/\s+/)
    .map((size) => {
      if (size.endsWith("fr")) {
        const frValue = Number.parseFloat(size.slice(0, -2));
        return Number.isNaN(frValue) ? 0 : { fr: frValue };
      }
      return parseLengthUnit(size) ?? 0;
    });

  return [{ repeat: [repetitionValue, sizes] }];
}

function parseSimpleTrackSizes(trackFunction: string): TrackSizingFunction[] {
  const parts = trackFunction.trim().split(/\s+/);
  return parts.map((part) => {
    if (part.endsWith("fr")) {
      const frValue = Number.parseFloat(part.slice(0, -2));
      return { single: Number.isNaN(frValue) ? 0 : { fr: frValue } };
    }
    return { single: parseLengthUnit(part) ?? 0 };
  });
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
  value: string | number | TrackSizingFunction[],
): TrackSizingFunction[] {
  if (typeof value === "string") {
    return parseTrackSizingFunction(value);
  }
  if (Array.isArray(value)) {
    return value as TrackSizingFunction[];
  }
  return typeof value === "number"
    ? ([{ single: value }] as TrackSizingFunction[])
    : ([{ single: 0 }] as TrackSizingFunction[]);
}

export function parseGridTemplateRows(
  value: string | number | TrackSizingFunction[],
): TrackSizingFunction[] {
  if (typeof value === "string") {
    return parseTrackSizingFunction(value);
  }
  if (Array.isArray(value)) {
    return value as TrackSizingFunction[];
  }
  return typeof value === "number"
    ? ([{ single: value }] as TrackSizingFunction[])
    : ([{ single: 0 }] as TrackSizingFunction[]);
}

export function parseLineClamp(value: string | number): number {
  if (value === "none") return 0;
  return Number(value);
}
