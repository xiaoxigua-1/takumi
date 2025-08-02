import type { Color } from "../types";

function parseHexColor(color: string): Color {
  const parsed = Number.parseInt(color.slice(1), 16);

  if (Number.isNaN(parsed)) {
    throw new Error(`Invalid hex color: ${color}`);
  }

  if (color.length === 4) {
    const r = (parsed >> 8) & 0xf;
    const g = (parsed >> 4) & 0xf;
    const b = parsed & 0xf;
    return (r << 20) | (r << 16) | (g << 12) | (g << 8) | (b << 4) | b;
  }

  if (color.length === 5) {
    const r = (parsed >> 12) & 0xf;
    const g = (parsed >> 8) & 0xf;
    const b = (parsed >> 4) & 0xf;
    const a = parsed & 0xf;
    return [r << 4 | r, g << 4 | g, b << 4 | b, a / 15] as Color;
  }

  if (color.length === 7) {
    return parsed;
  }

  if (color.length === 9) {
    return [
      (parsed >> 24) & 0xff, // Red
      (parsed >> 16) & 0xff, // Green
      (parsed >> 8) & 0xff, // Blue
      (parsed & 0xff) / 0xff, // Alpha
    ] as Color;
  }

  throw new Error(`Invalid hex color: ${color}`);
}

function parseRgbColor(color: string): Color {
  const args = color
    .slice(4, -1)
    .split(",")
    .map((arg) => Number.parseInt(arg.trim()).toString(16).padStart(2, "0"));

  if (args.length !== 3) {
    throw new Error(`Invalid rgb color: ${color}`);
  }

  return Number.parseInt(args.join(""), 16) as Color;
}

function parseRgbaColor(color: string): Color {
  const args = color
    .slice(5, -1)
    .split(",")
    .map((arg) => Number.parseFloat(arg.trim()));

  if (args.length !== 4) {
    throw new Error(`Invalid rgba color: ${color}`);
  }

  const hex = args as [number, number, number, number];

  if (
    hex[0] < 0 ||
    hex[0] > 255 ||
    hex[1] < 0 ||
    hex[1] > 255 ||
    hex[2] < 0 ||
    hex[2] > 255 ||
    hex[3] < 0 ||
    hex[3] > 1
  ) {
    throw new Error(`Invalid rgba color: ${color}`);
  }

  return hex;
}

export function parseColor(color: string): Color {
  if (color.startsWith("#")) {
    return parseHexColor(color);
  }

  if (color.startsWith("rgb(")) {
    return parseRgbColor(color);
  }

  if (color.startsWith("rgba(")) {
    return parseRgbaColor(color);
  }

  throw new Error(`Invalid color: ${color}`);
}
