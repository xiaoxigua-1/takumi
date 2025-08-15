import type { Style } from "./bindings/Style";

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

export type PartialStyle = Partial<Style>;

export type Node = ContainerNode | TextNode | ImageNode | AnyNode;

export type ContainerNode = PartialStyle & {
  type: "container";
  children?: Node[];
};

export type TextNode = PartialStyle & {
  type: "text";
  text: string;
};

export type ImageNode = PartialStyle & {
  type: "image";
  src: string;
};
