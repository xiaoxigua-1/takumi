import type { Property } from "csstype";
import type { CSSProperties } from "react";
import { parseColor } from "./color-parsing";
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
} from "./types";
import { em, percentage, rem, vh, vw } from "./helpers";

export function parseLengthUnit(
  value: Property.Width | number,
): LengthUnit | undefined {
  if (typeof value === "number") return value;

  if (
    value === "auto" ||
    value === "min-content" ||
    value === "max-content" ||
    value === "fit-content"
  )
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
): SidesValue<LengthUnit> | undefined {
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

export function parseAspectRatio(
  aspectRatio: Property.AspectRatio,
): number | undefined {
  if (typeof aspectRatio === "number") return aspectRatio;

  if (aspectRatio.includes("/")) {
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

  return Number.parseFloat(aspectRatio);
}

export function parseDisplay(display: Property.Display): Display | undefined {
  switch (display) {
    case "block":
    case "flex":
    case "grid":
      return display as Display;
    default: {
      console.warn(`Invalid display value: ${display}, fallback to undefined.`);
    }
  }
}

export function parsePosition(
  position: Property.Position,
): Position | undefined {
  switch (position) {
    case "static":
    case "relative":
    case "absolute":
      return position as Position;
    default: {
      console.warn(
        `Invalid position value: ${position}, fallback to undefined.`,
      );
    }
  }
}

export function parseBoxShadow(
  source: Property.BoxShadow,
): BoxShadow | undefined {
  const parts = source.trim().split(/\s+/);
  if (parts.length < 2) return;

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

  if (lengthValues.length < 2 || lengthValues.length > 4) return;

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
    return;
  }
}

export function parseGridTrackSize(
  trackSize: string,
): GridTrackSize[] | undefined {
  const parts = trackSize.trim().split(/\s+/);
  return parts.map((part) => {
    if (part.endsWith("fr")) {
      const frValue = Number.parseFloat(part.slice(0, -2));
      if (Number.isNaN(frValue)) return 0;
      return { fr: frValue };
    }
    return parseLengthUnit(part) ?? 0;
  });
}

export function parseGridAutoFlow(
  autoFlow: Property.GridAutoFlow,
): GridAutoFlow | undefined {
  switch (autoFlow) {
    case "row":
    case "column":
    case "row dense":
    case "column dense":
      return autoFlow.replace(" ", "-") as GridAutoFlow;
    default:
      console.warn(
        `Invalid grid-auto-flow value: ${autoFlow}, fallback to undefined.`,
      );
      return;
  }
}

export function parseGridLine(gridLine: string): GridLine {
  if (gridLine.startsWith("span ")) {
    const spanValue = Number.parseInt(gridLine.slice(5));
    return Number.isNaN(spanValue)
      ? undefined
      : { start: spanValue, end: null };
  }

  if (gridLine.includes("/")) {
    const parts = gridLine.split("/");
    if (parts.length !== 2) return;

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
  return Number.isNaN(lineNumber)
    ? { start: gridLine, end: null }
    : { start: lineNumber, end: null };
}

export function parseTrackSizingFunction(
  trackFunction: string,
): TrackSizingFunction[] | undefined {
  const repeatMatch = trackFunction.match(/repeat\(([^,]+),\s*(.+)\)/);
  if (repeatMatch) {
    return parseRepeatFunction(repeatMatch);
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
): TextOverflow | undefined {
  switch (textOverflow) {
    case "ellipsis":
    case "clip":
      return textOverflow as TextOverflow;
    default:
      console.warn(
        `Invalid text-overflow value: ${textOverflow}, fallback to undefined.`,
      );
      return;
  }
}

export function parseFontWeight(
  fontWeight: Property.FontWeight,
): FontWeight | undefined {
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

  console.warn(
    `Invalid font-weight value: ${fontWeight}, fallback to undefined.`,
  );
}
