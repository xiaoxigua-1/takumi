import type { JSX } from "react";
import { em, rgba } from "../helpers";
import type { PartialStyle } from "../types";

// Reference from Chromium's default style presets
// https://chromium.googlesource.com/chromium/blink/+/master/Source/core/css/html.css
// https://github.com/vercel/satori/blob/main/src/handler/presets.ts
export const stylePresets: Partial<
  Record<keyof JSX.IntrinsicElements, PartialStyle>
> = {
  // Generic block-level elements
  p: {
    margin: [em(1), 0],
  },
  blockquote: {
    margin: [em(1), 40],
  },
  center: {
    textAlign: "center",
  },
  hr: {
    margin: [em(0.5), "auto"],
    borderWidth: 1,
  },
  // Heading elements
  h1: {
    fontSize: em(2),
    margin: [em(0.67), 0],
    fontWeight: 700,
  },
  h2: {
    fontSize: em(1.5),
    margin: [em(0.83), 0],
    fontWeight: 700,
  },
  h3: {
    fontSize: em(1.17),
    margin: [em(1), 0],
    fontWeight: 700,
  },
  h4: {
    margin: [em(1.33), 0],
    fontWeight: 700,
  },
  h5: {
    fontSize: em(0.83),
    margin: [em(1.67), 0],
    fontWeight: 700,
  },
  h6: {
    fontSize: em(0.67),
    margin: [em(2.33), 0],
    fontWeight: 700,
  },
  // Inline elements
  strong: {
    fontWeight: 700,
  },
  b: {
    fontWeight: 700,
  },
  code: {
    fontFamily: "monospace",
  },
  kbd: {
    fontFamily: "monospace",
  },
  pre: {
    fontFamily: "monospace",
    margin: [em(1), 0],
  },
  mark: {
    backgroundColor: rgba(255, 255, 0),
    color: 0,
  },
  big: {
    fontSize: em(1.2),
  },
  small: {
    fontSize: em(1 / 1.2),
  },
};
