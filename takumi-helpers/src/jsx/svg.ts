import type { ComponentProps, ReactElement } from "react";
import { isReactElement } from "./utils";

function isTextNode(node: unknown): node is string | number {
  return typeof node === "string" || typeof node === "number";
}

function escapeAttr(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/"/g, "&quot;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function styleObjectToString(styleObj: Record<string, unknown>): string {
  return Object.keys(styleObj)
    .map(
      (k) =>
        `${k.replace(/([A-Z])/g, "-$1").toLowerCase()}:${String(styleObj[k]).trim()}`,
    )
    .join(";");
}

const svgKebabAttrs = [
  "stop-color",
  "stop-opacity",
  "stroke-width",
  "stroke-dasharray",
  "stroke-dashoffset",
  "stroke-linecap",
  "stroke-linejoin",
  "fill-rule",
  "clip-rule",
  "color-interpolation-filters",
  "flood-color",
  "flood-opacity",
  "accent-height",
  "alignment-baseline",
  "arabic-form",
  "baseline-shift",
  "cap-height",
  "clip-path",
  "clip-path-units",
  "color-interpolation",
  "color-profile",
  "color-rendering",
  "enable-background",
  "fill-opacity",
  "font-family",
  "font-size",
  "font-size-adjust",
  "font-stretch",
  "font-style",
  "font-variant",
  "font-weight",
  "glyph-name",
  "glyph-orientation-horizontal",
  "glyph-orientation-vertical",
  "horiz-adv-x",
  "horiz-origin-x",
  "image-rendering",
  "letter-spacing",
  "lighting-color",
  "marker-end",
  "marker-mid",
  "marker-start",
  "overline-position",
  "overline-thickness",
  "paint-order",
  "preserve-aspect-ratio",
  "pointer-events",
  "shape-rendering",
  "stroke-miterlimit",
  "stroke-opacity",
  "text-anchor",
  "text-decoration",
  "text-rendering",
  "transform-origin",
  "underline-position",
  "underline-thickness",
  "unicode-bidi",
  "unicode-range",
  "units-per-em",
  "vector-effect",
  "vert-adv-y",
  "vert-origin-x",
  "vert-origin-y",
  "v-alphabetic",
  "v-hanging",
  "v-ideographic",
  "v-mathematical",
  "word-spacing",
  "writing-mode",
];

const svgAttrMap: Record<string, string> = Object.fromEntries(
  svgKebabAttrs.map((kebab) => {
    const camel = kebab.replace(/-([a-z])/g, (_m, c) => c.toUpperCase());
    return [camel, kebab];
  }),
);

function serializePropToAttrString(
  key: string,
  value: unknown,
): string | undefined {
  if (key === "children" || value == null) return;

  // Determine the final attribute name (handle className and known SVG mappings)
  let attrName: string;
  if (key === "className") {
    attrName = "class";
  } else if (svgAttrMap[key]) {
    attrName = svgAttrMap[key];
  } else {
    const possibleKebab = key.replace(/([A-Z])/g, "-$1").toLowerCase();
    attrName = svgKebabAttrs.includes(possibleKebab) ? possibleKebab : key;
  }

  if (typeof value === "boolean") {
    // For SVG serialization we want boolean attributes to be explicit like
    // `focusable="true"` to match react-dom server output.
    return `${attrName}="${String(value)}"`;
  }

  if (key === "style" && typeof value === "object") {
    const styleString = styleObjectToString(value as Record<string, unknown>);
    if (styleString) return `style="${escapeAttr(styleString)}"`;
  }

  return `${attrName}="${escapeAttr(String(value))}"`;
}

function propsToAttrStrings(props: Record<string, unknown>): string[] {
  return Object.entries(props)
    .map(([key, value]) => serializePropToAttrString(key, value))
    .filter((attr): attr is string => attr !== undefined);
}

const serializeElementNode = (
  obj: ReactElement,
  serializeFn: (n: unknown) => string,
): string => {
  const props = (obj.props as Record<string, unknown>) || {};

  const attrs = propsToAttrStrings(props);
  const children = props.children;
  const childrenString = Array.isArray(children)
    ? children.map((c) => serializeFn(c)).join("")
    : serializeFn(children);

  return `<${obj.type}${attrs.length > 0 ? ` ${attrs.join(" ")}` : ""}>${childrenString}</${obj.type}>`;
};

const serialize = (node: unknown): string => {
  if (node === null || node === undefined || node === false) return "";
  if (isTextNode(node)) return String(node);
  if (Array.isArray(node)) return node.map(serialize).join("");
  if (!isReactElement(node)) return "";

  return serializeElementNode(node, serialize);
};

export function serializeSvg(
  element: ReactElement<ComponentProps<"svg">>,
): string {
  const props = (element.props as Record<string, unknown>) || {};

  if (!("xmlns" in props)) {
    const cloned: ReactElement = {
      ...element,
      props: {
        ...props,
        xmlns: "http://www.w3.org/2000/svg",
      },
    } as ReactElement;

    return serialize(cloned) || "";
  }

  return serialize(element) || "";
}
