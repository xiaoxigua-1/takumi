import type { ColorInput } from "./types";

export function parseColor(color: string): ColorInput {
  if (color.startsWith("#")) {
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

  if (color.startsWith("rgb(")) {
    const rgbMatch = color.match(
      /^rgb\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)$/,
    );

    if (rgbMatch) {
      const [, r, g, b] = rgbMatch;
      return [Number(r), Number(g), Number(b)] as ColorInput;
    }
  }

  throw new Error(`Invalid RGB color: ${color}`);
}
