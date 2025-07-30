import type { JSX } from "react";
import { em, rgba } from "../helpers";
import type { Style } from "../types";

// Reference from Chromium's default style presets
// https://chromium.googlesource.com/chromium/blink/+/master/Source/core/css/html.css
// https://github.com/vercel/satori/blob/main/src/handler/presets.ts
export const stylePresets: Partial<
  Record<keyof JSX.IntrinsicElements, Partial<Style>>
> = {
  // Generic block-level elements
  p: {
    margin: [em(1), 0],
  },
  blockquote: {
    margin: [em(1), 40],
  },
  center: {
    text_align: "center",
  },
  hr: {
    margin: [em(0.5), "auto"],
    border_width: 1,
  },
  // Heading elements
  h1: {
    font_size: em(2),
    margin: [em(0.67), 0],
    font_weight: 700,
  },
  h2: {
    font_size: em(1.5),
    margin: [em(0.83), 0],
    font_weight: 700,
  },
  h3: {
    font_size: em(1.17),
    margin: [em(1), 0],
    font_weight: 700,
  },
  h4: {
    margin: [em(1.33), 0],
    font_weight: 700,
  },
  h5: {
    font_size: em(0.83),
    margin: [em(1.67), 0],
    font_weight: 700,
  },
  h6: {
    font_size: em(0.67),
    margin: [em(2.33), 0],
    font_weight: 700,
  },
  // Tables
  // Lists
  // Form elements
  // Inline elements
  strong: {
    font_weight: 700,
  },
  b: {
    font_weight: 700,
  },
  code: {
    font_family: "monospace",
  },
  kbd: {
    font_family: "monospace",
  },
  pre: {
    font_family: "monospace",
    margin: [em(1), 0],
  },
  mark: {
    background_color: rgba(255, 255, 0),
    color: 0,
  },
  big: {
    font_size: 16 * 1.2,
  },
  small: {
    font_size: 16 / 1.2,
  },
};
