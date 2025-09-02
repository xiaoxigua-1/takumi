import type { ComponentProps, CSSProperties, JSX, ReactNode } from "react";
import { container, image, percentage, text } from "../helpers";
import type { Node, PartialStyle } from "../types";
import { stylePresets } from "./style-presets";
import { serializeSvg } from "./svg";
import {
  isFunctionComponent,
  isHtmlElement,
  isReactForwardRef,
  isReactMemo,
  isValidElement,
} from "./utils";

export type ReactElementLike<
  K = string | ((props: unknown) => ReactNode) | symbol,
  P = unknown,
> = {
  type: K;
  props: P;
};

export async function fromJsx(
  element: ReactNode | ReactElementLike,
): Promise<Node> {
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

async function fromJsxInternal(
  element: ReactNode | ReactElementLike,
): Promise<Node[]> {
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

function tryHandleForwardRef(
  element: ReactElementLike,
): Promise<Node[]> | undefined {
  if (typeof element.type !== "object" || element.type === null)
    return undefined;

  // Check if this is a forwardRef component
  if (isReactForwardRef(element.type) && "render" in element.type) {
    const forwardRefType = element.type as {
      render: (props: unknown, ref: unknown) => ReactNode;
    };
    return fromJsxInternal(forwardRefType.render(element.props, null));
  }
}

function tryHandleMemo(element: ReactElementLike): Promise<Node[]> | undefined {
  if (typeof element.type !== "object" || element.type === null)
    return undefined;

  // Check if this is a memo component
  if (isReactMemo(element.type) && "type" in element.type) {
    const memoType = element.type as { type: unknown };
    const innerType = memoType.type;

    if (isFunctionComponent(innerType)) {
      return fromJsxInternal(innerType(element.props));
    }

    const cloned: ReactElementLike = {
      ...element,
      type: innerType as ReactElementLike["type"],
    } as ReactElementLike;

    return processReactElement(cloned);
  }
}

async function processReactElement(element: ReactElementLike): Promise<Node[]> {
  if (isFunctionComponent(element.type)) {
    return fromJsxInternal(element.type(element.props));
  }

  const forwardRefResult = tryHandleForwardRef(element);
  if (forwardRefResult !== undefined) return forwardRefResult;

  const memoResult = tryHandleMemo(element);
  if (memoResult !== undefined) return memoResult;

  // Handle React fragments <></>
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
  const style = extractStyleFromProps(element.props) as PartialStyle;

  if (typeof element.type === "string" && element.type in stylePresets) {
    Object.assign(
      style,
      stylePresets[element.type as keyof JSX.IntrinsicElements],
    );
  }

  return [
    container({
      children,
      style,
    }),
  ];
}

function createImageElement(
  element: ReactElementLike<"img", ComponentProps<"img">>,
) {
  if (!element.props.src) {
    throw new Error("Image element must have a 'src' prop.");
  }

  const style = extractStyleFromProps(element.props) as PartialStyle;

  return image({
    src: element.props.src,
    style,
  });
}

function createSvgElement(
  element: ReactElementLike<"svg", ComponentProps<"svg">>,
) {
  const style = extractStyleFromProps(element.props) as PartialStyle;

  const svg = serializeSvg(element);

  return image({
    src: svg,
    style,
  });
}

function extractStyleFromProps(props: unknown): CSSProperties {
  if (typeof props !== "object" || props === null) return {};

  return "style" in props && typeof props.style === "object"
    ? (props.style as CSSProperties)
    : {};
}

function collectChildren(element: ReactElementLike): Promise<Node[]> {
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
