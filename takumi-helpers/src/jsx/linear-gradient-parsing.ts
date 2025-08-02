import type { Property } from "csstype";
import type { Color, Gradient, GradientStop } from "../types";
import { parseColor } from "./color-parsing";

/**
 * Parse CSS-like linear-gradient(...) string into Gradient { angle, stops }
 * Supported:
 * - linear-gradient(to right|left|top|bottom|..., color [pos]?, color [pos]?, ...)
 * - linear-gradient(<angle>deg|rad|turn, ...)
 * - color stops with optional positions as percentages (e.g. 0%, 50%, 100%) or unitless 0..1
 * Notes:
 * - Angle is normalized to 0..360 degrees
 * - If no angle/direction provided, defaults to 180deg (CSS 'to bottom')
 * - If a stop omits position, positions are auto-distributed evenly or interpolated
 */
export function parseLinearGradient(
  value: Property.BackgroundImage,
): Gradient | undefined {
  if (value === "none") return;

  const fnMatch = value.match(/^linear-gradient\((.*)\)$/i)?.[1];

  if (!fnMatch) {
    throw new Error(`Invalid linear-gradient syntax: ${value}`);
  }

  const inner = fnMatch.trim();
  const args = splitTopLevel(inner, ",");

  const firstArg = args[0]?.trim();

  if (!firstArg) {
    throw new Error(`linear-gradient requires at least one argument: ${value}`);
  }

  const angle = parseAngleOrDirectionArg(firstArg);
  const effectiveAngle = angle !== undefined ? normalizeAngle(angle) : 180;
  const stopArgs = angle !== undefined ? args.slice(1) : args;

  if (stopArgs.length < 2) {
    throw new Error(
      `linear-gradient requires at least two color stops: ${value}`,
    );
  }

  const parsedStops = stopArgs.map(parseStop);

  return {
    angle: effectiveAngle,
    stops: finalizeStops(parsedStops),
  };
}

function splitTopLevel(input: string, separator: string): string[] {
  const parts: string[] = [];

  let depth = 0;
  let start = 0;

  for (let i = 0; i < input.length; i++) {
    const char = input[i];

    if (char === "(") {
      depth++;
    } else if (char === ")") {
      depth = Math.max(0, depth - 1);
    }

    if (depth === 0 && input.startsWith(separator, i)) {
      const part = input.slice(start, i).trim();
      if (part) parts.push(part);

      start = i + separator.length;
      i += separator.length - 1;
    }
  }

  const lastPart = input.slice(start).trim();
  if (lastPart) parts.push(lastPart);

  return parts;
}

function parseAngleOrDirectionArg(arg: string) {
  const trimmed = arg.trim();

  return parseDirection(trimmed) ?? parseAngle(trimmed);
}

function parseDirection(directionArgument: string): number | undefined {
  const match = directionArgument.match(
    /^to\s+(top|bottom|left|right)(?:\s+(top|bottom|left|right))?$/i,
  );
  if (!match) return undefined;

  const primary = (match[1] ?? "").toLowerCase();
  const secondary = (match[2] ?? "").toLowerCase();

  const map: Record<string, number> = {
    top: 0,
    right: 90,
    bottom: 180,
    left: 270,
  };
  const primaryAngle = map[primary] ?? 180;
  if (!secondary) return primaryAngle;

  const secondaryAngle = map[secondary] ?? 180;
  const set = new Set([primaryAngle, secondaryAngle]);
  if (set.has(0) && set.has(90)) return 45;
  if (set.has(90) && set.has(180)) return 135;
  if (set.has(180) && set.has(270)) return 225;
  if (set.has(270) && set.has(0)) return 315;
  return normalizeAngle((primaryAngle + secondaryAngle) / 2);
}

function parseAngle(angleArgument: string): number | undefined {
  const match = angleArgument.match(/^(-?\d*\.?\d+)\s*(deg|rad|turn)?$/i);

  if (!match) return undefined;

  const value = Number.parseFloat(match[1] ?? "0");
  const unit = match[2] ? match[2].toLowerCase() : "deg";

  switch (unit) {
    case "deg":
      return value;
    case "rad":
      return (value * 180) / Math.PI;
    case "turn":
      return value * 360;
    default:
      return value;
  }
}

type ParsedStop = {
  color: Color;
  position?: number;
};

function parseStop(stopString: string): ParsedStop {
  const tokens = splitTopLevel(stopString.trim(), " ");

  const color = parseColor(tokens[0] ?? stopString.trim());
  const position = tokens[1] ? parsePositionToken(tokens[1]) : undefined;

  return {
    color,
    position,
  };
}

function parsePositionToken(positionToken: string): number {
  const trimmed = positionToken.trim();

  if (trimmed.endsWith("%")) {
    return Number.parseFloat(trimmed.slice(0, -1)) / 100;
  }

  return Number.parseFloat(trimmed);
}

function finalizeStops(parsedStops: ParsedStop[]): GradientStop[] {
  if (parsedStops.every((stop) => stop.position === undefined)) {
    return evenlyDistribute(parsedStops);
  }

  const stops: GradientStop[] = [];
  const stopCount = parsedStops.length;

  const firstStop = parsedStops[0];

  if (!firstStop) {
    throw new Error("First stop must have a position defined");
  }

  // First stop at 0
  stops.push({
    color: firstStop.color,
    position: 0,
  });

  // Interior stops with explicit positions
  for (let i = 1; i < stopCount - 1; i++) {
    const currentStop = parsedStops[i];

    if (typeof currentStop?.position === "number") {
      stops.push({
        color: currentStop.color,
        position: clamp01(currentStop.position),
      });
    }
  }

  const lastStop = parsedStops[stopCount - 1];

  if (!lastStop) {
    throw new Error("Last stop must have a position defined");
  }

  // Last stop at 1
  stops.push({ color: lastStop.color, position: 1 });

  // Sort and deduplicate
  const deduped = stops
    .reduce<GradientStop[]>((acc, curr) => {
      if (acc.length === 0 || acc[acc.length - 1]?.position !== curr.position) {
        acc.push(curr);
      }
      return acc;
    }, [])
    .sort((a, b) => a.position - b.position);

  return deduped;
}

function evenlyDistribute(parsedStops: ParsedStop[]): GradientStop[] {
  const stopCount = parsedStops.length;
  return parsedStops.map((stop, index) => ({
    color: stop.color,
    position: stopCount === 1 ? 0 : index / (stopCount - 1),
  }));
}

function clamp01(value: number): number {
  return Math.max(0, Math.min(1, value));
}

function normalizeAngle(angleDegrees: number): number {
  let normalized = angleDegrees % 360;
  if (normalized < 0) normalized += 360;

  return normalized;
}
