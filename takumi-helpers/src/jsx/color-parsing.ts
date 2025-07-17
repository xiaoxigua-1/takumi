import type { ColorInput } from "../types";

function parseHexColor(color: string): ColorInput {
  if (color.length === 7)
    return Number.parseInt(color.slice(1), 16) as ColorInput;

  if (color.length === 9) {
    const hex = Number.parseInt(color.slice(1), 16);

    return [
      (hex >> 24) & 0xff, // Red
      (hex >> 16) & 0xff, // Green
      (hex >> 8) & 0xff, // Blue
      (hex & 0xff) / 0xff, // Alpha
    ] as ColorInput;
  }

  throw new Error(`Invalid hex color: ${color}`);
}

function parseRgbColor(color: string): ColorInput | null {
  const rgbMatch = color.match(/^rgb\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)$/);

  if (rgbMatch) {
    const [, r, g, b] = rgbMatch;
    return [Number(r), Number(g), Number(b)] as ColorInput;
  }

  return null;
}

function parseRgbaColor(color: string): ColorInput | null {
  const rgbaMatch = color.match(
    /^rgba\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*([0-9]*\.?[0-9]+)\s*\)$/,
  );

  if (rgbaMatch) {
    const [, r, g, b, a] = rgbaMatch;
    const red = Number(r);
    const green = Number(g);
    const blue = Number(b);
    const alpha = Number(a);

    // Validate RGB values are in range 0-255
    if (
      red < 0 ||
      red > 255 ||
      green < 0 ||
      green > 255 ||
      blue < 0 ||
      blue > 255
    ) {
      return null;
    }

    // Validate alpha is in range 0-1
    if (alpha < 0 || alpha > 1) {
      return null;
    }

    return [red, green, blue, alpha] as ColorInput;
  }

  return null;
}

export function parseColor(color: string): ColorInput {
  if (color.startsWith("#")) {
    return parseHexColor(color);
  }

  if (color.startsWith("rgb(")) {
    const result = parseRgbColor(color);
    if (result !== null) {
      return result;
    }
  }

  if (color.startsWith("rgba(")) {
    const result = parseRgbaColor(color);
    if (result !== null) {
      return result;
    }
  }

  throw new Error(`Invalid RGB color: ${color}`);
}
