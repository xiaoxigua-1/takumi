import {
  type ComponentProps,
  type CSSProperties,
  cloneElement,
  type JSX,
  type ReactElement,
  type ReactNode,
} from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { container, image, text } from "../helpers";
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

export async function fromJsx(element: ReactNode): Promise<Node[]> {
  if (element === undefined || element === null) return [];

  // If element is a server component, wait for it to resolve first
  if (element instanceof Promise) return fromJsx(await element);

  //
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
    return fromJsx(FunctionComponent(element.props));
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
    return createImageElement(element);
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

  const width = element.props.style?.width ?? element.props.width;
  const height = element.props.style?.height ?? element.props.height;

  return [
    image(element.props.src, {
      ...(element.props.style as PartialStyle),
      width,
      height,
    }),
  ];
}

function createSvgElement(element: ReactElement<ComponentProps<"svg">>) {
  return image(
    renderToStaticMarkup(
      cloneElement(
        element,
        {
          xmlns: "http://www.w3.org/2000/svg",
          ...element.props,
        },
        element.props.children,
      ),
    ),
  );
}

function extractStyleFromProps(props: unknown): CSSProperties | undefined {
  return typeof props === "object" && props && "style" in props
    ? (props.style as CSSProperties)
    : undefined;
}

function isHtmlElement<T extends keyof JSX.IntrinsicElements>(
  element: ReactElement,
  type: T,
): element is ReactElement<ComponentProps<T>> {
  return element.type === type;
}

async function collectChildren(element: ReactElement): Promise<Node[]> {
  if (
    typeof element.props !== "object" ||
    element.props === null ||
    !("children" in element.props)
  )
    return [];

  const result = await fromJsx(element.props.children as ReactNode);
  return result;
}

async function collectIterable(iterable: Iterable<ReactNode>): Promise<Node[]> {
  const children = await Promise.all(Array.from(iterable).map(fromJsx));

  return children.flat();
}
