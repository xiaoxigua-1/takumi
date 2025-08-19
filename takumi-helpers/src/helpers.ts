import type { Color } from "./bindings/Color";
import type { ContainerNode, ImageNode, PartialStyle, TextNode } from "./types";

export function container(props: Omit<ContainerNode, "type">): ContainerNode {
  return {
    type: "container",
    ...props,
  };
}

export function text(text: string, style?: PartialStyle): TextNode {
  const node: TextNode = {
    type: "text",
    text,
  };

  if (style) {
    node.style = style;
  }

  return node;
}

export function image(props: Omit<ImageNode, "type">): ImageNode {
  return {
    type: "image",
    ...props,
  };
}

export function style(style: PartialStyle) {
  return style;
}

/**
 * Convert a number to a percentage struct.
 * @param percentage - The percentage to convert (0.0 - 100.0).
 * @returns The percentage struct.
 */
export function percentage(percentage: number) {
  return {
    percentage,
  };
}

export function vw(vw: number) {
  return {
    vw,
  };
}

export function vh(vh: number) {
  return {
    vh,
  };
}

export function em(em: number) {
  return {
    em,
  };
}

export function rem(rem: number) {
  return {
    rem,
  };
}

export function fr(fr: number) {
  return {
    fr,
  };
}

export function rgba(r: number, g: number, b: number, a = 1): Color {
  return [r, g, b, a];
}
