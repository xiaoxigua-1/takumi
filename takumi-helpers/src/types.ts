import type { Style } from "./bindings/Style";

// Export all binding types
export * from "./bindings/AlignItems";
export * from "./bindings/BoxShadow";
export * from "./bindings/BoxShadowInput";
export * from "./bindings/Color";
export * from "./bindings/ColorInput";
export * from "./bindings/Display";
export * from "./bindings/FlexDirection";
export * from "./bindings/FlexWrap";
export * from "./bindings/FontFamily";
export * from "./bindings/FontWeight";
export * from "./bindings/Gap";
export * from "./bindings/Gradient";
export * from "./bindings/GradientStop";
export * from "./bindings/GridAutoFlow";
export * from "./bindings/GridLengthUnit";
export * from "./bindings/GridLine";
export * from "./bindings/GridMinMaxSize";
export * from "./bindings/GridPlacement";
export * from "./bindings/GridRepeatTrack";
export * from "./bindings/GridRepetitionCount";
export * from "./bindings/GridTemplateComponent";
export * from "./bindings/GridTrackSize";
export * from "./bindings/ImageScalingAlgorithm";
export * from "./bindings/InheritableStyle";
export * from "./bindings/JustifyContent";
export * from "./bindings/LengthUnit";
export * from "./bindings/ObjectFit";
export * from "./bindings/Position";
export * from "./bindings/SidesValue";
export * from "./bindings/Style";
export * from "./bindings/TextAlign";
export * from "./bindings/TextOverflow";

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

export type JsxParsedStyle = {
  [key in keyof Style]: Style[key] | undefined;
};

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