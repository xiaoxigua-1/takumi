import {
  type ComponentProps,
  type CSSProperties,
  cloneElement,
  type JSX,
  type ReactElement,
  type ReactNode,
} from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { container, image, percentage, text } from "../helpers";
import type { Node, PartialStyle } from "../types";
import { stylePresets } from "./style-presets";

const REACT_ELEMENT_TYPE = Symbol.for("react.transitional.element");

function isValidElement(object: unknown): object is ReactElement {
  return (
    typeof object === "object" &&
    object !== null &&
    "$$typeof" in object &&
    object.$$typeof === REACT_ELEMENT_TYPE
  );
}

export async function fromJsx(element: ReactNode): Promise<Node> {
  const result = await fromJsxInternal(element);

  if (result.length === 0) {
    return container({});
  }

  if (result.length === 1 && result[0] !== undefined) {
    return result[0];
  }

  return container({
    children: result,
    style: {
      width: percentage(100),
      height: percentage(100),
    },
  });
}

async function fromJsxInternal(element: ReactNode): Promise<Node[]> {
  if (element === undefined || element === null) return [];

  // If element is a server component, wait for it to resolve first
  if (element instanceof Promise) return fromJsxInternal(await element);

  // If element is an iterable, collect the children
  if (typeof element === "object" && Symbol.iterator in element)
    return collectIterable(element);

  if (isValidElement(element)) {
    const result = await processReactElement(element);
    return Array.isArray(result) ? result : result ? [result] : [];
  }

  return [text(String(element))];
}

async function processReactElement(element: ReactElement): Promise<Node[]> {
  if (typeof element.type === "function") {
    const FunctionComponent = element.type as (props: unknown) => ReactNode;
    return fromJsxInternal(FunctionComponent(element.props));
  }

  // Handle React fragments
  if (
    typeof element.type === "symbol" &&
    element.type === Symbol.for("react.fragment")
  ) {
    const children = await collectChildren(element);
    return children || [];
  }

  if (isHtmlElement(element, "img")) {
    return [createImageElement(element)];
  }

  if (isHtmlElement(element, "svg")) {
    return [createSvgElement(element)];
  }

  const children = await collectChildren(element);
  const style = extractStyleFromProps(element.props);

  return [
    container({
      children,
      ...stylePresets[element.type as keyof JSX.IntrinsicElements],
      ...(style as PartialStyle),
    }),
  ];
}

function createImageElement(element: ReactElement<ComponentProps<"img">>) {
  if (!element.props.src) {
    throw new Error("Image element must have a 'src' prop.");
  }

  const style = extractStyleFromProps(element.props);

  return image({
    src: element.props.src,
    style: style as PartialStyle,
  });
}

function createSvgElement(element: ReactElement<ComponentProps<"svg">>) {
  const style = extractStyleFromProps(element.props);

  const svg = renderToStaticMarkup(
    cloneElement(
      element,
      {
        xmlns: "http://www.w3.org/2000/svg",
        ...element.props,
      },
      element.props.children,
    ),
  );

  return image({
    src: svg,
    style: style as PartialStyle,
  });
}

function extractStyleFromProps(props: unknown): CSSProperties | undefined {
  if (typeof props !== "object" || props === null) return undefined;

  const style: CSSProperties =
    "style" in props ? (props.style as CSSProperties) : {};

  const width =
    "width" in props &&
    (typeof props.width === "number" || typeof props.width === "string")
      ? props.width
      : undefined;

  const height =
    "height" in props &&
    (typeof props.height === "number" || typeof props.height === "string")
      ? props.height
      : undefined;

  if (width !== undefined && !("width" in style)) style.width = width;
  if (height !== undefined && !("height" in style)) style.height = height;

  return style;
}

function isHtmlElement<T extends keyof JSX.IntrinsicElements>(
  element: ReactElement,
  type: T,
): element is ReactElement<ComponentProps<T>> {
  return element.type === type;
}

function collectChildren(element: ReactElement): Promise<Node[]> {
  if (
    typeof element.props !== "object" ||
    element.props === null ||
    !("children" in element.props)
  )
    return Promise.resolve([]);

  return fromJsxInternal(element.props.children as ReactNode);
}

async function collectIterable(iterable: Iterable<ReactNode>): Promise<Node[]> {
  const children = await Promise.all(Array.from(iterable).map(fromJsxInternal));

  return children.flat();
}
