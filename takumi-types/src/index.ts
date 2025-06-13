import type { Style } from "./bindings/Style";

export * from "./bindings/AlignItems";
export * from "./bindings/Color";
export * from "./bindings/ColorInput";
export * from "./bindings/FlexDirection";
export * from "./bindings/FontWeight";
export * from "./bindings/Gap";
export * from "./bindings/Gradient";
export * from "./bindings/JustifyContent";
export * from "./bindings/ObjectFit";
export * from "./bindings/Position";
export * from "./bindings/SidesValue";
export * from "./bindings/Style";
export * from "./bindings/TextAlign";
export * from "./bindings/ValuePercentageAuto";

type JsonValue =
  | string
  | number
  | boolean
  | null
  | JsonValue[]
  | { [key: string]: JsonValue };

type AnyNode = {
  type: string;
  [key: string]: JsonValue;
};

export type Node = ContainerNode | TextNode | ImageNode | AnyNode;

export type ContainerNode = {
  type: "container";
  style: Style;
  children: Node[];
};

export type TextNode = {
  type: "text";
  style: Style;
  text: string;
};

export type ImageNode = {
  type: "image";
  style: Style;
  src: string;
};

export function container(style: Style, children: Node[]): ContainerNode {
  return {
    type: "container",
    style,
    children,
  };
}

export function text(style: Style, text: string): TextNode {
  return {
    type: "text",
    style,
    text,
  };
}

export function image(style: Style, src: string): ImageNode {
  return {
    type: "image",
    style,
    src,
  };
}

export function style(style: Style): Style {
  return style;
}
