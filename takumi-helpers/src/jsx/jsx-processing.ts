import type {
  ComponentProps,
  CSSProperties,
  JSX,
  ReactElement,
  ReactNode,
} from "react";
import { container, image, text } from "../helpers";
import type { Node } from "../types";
import { parseStyle } from "./style-parser";

type Nodes = Node | Nodes[];

const REACT_ELEMENT_TYPE = Symbol.for("react.transitional.element");

function isValidElement(object: unknown): object is ReactElement {
  return (
    typeof object === "object" &&
    object !== null &&
    "$$typeof" in object &&
    object.$$typeof === REACT_ELEMENT_TYPE
  );
}

function flattenNodes(nodes: Nodes[]): Node[] {
  const result: Node[] = [];
  for (const node of nodes) {
    if (Array.isArray(node)) {
      result.push(...flattenNodes(node));
    } else {
      result.push(node);
    }
  }
  return result;
}

export async function fromJsx(element: ReactNode): Promise<Nodes | undefined> {
  if (element === undefined || element === null) return;

  if (element instanceof Promise) return fromJsx(await element);

  if (typeof element === "object" && Symbol.iterator in element)
    return collectIterable(element);

  if (isValidElement(element)) {
    return processReactElement(element);
  }

  return text(String(element));
}

async function processReactElement(
  element: ReactElement,
): Promise<Nodes | undefined> {
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

  if (isHtmlElement(element, "img") && element.props.src) {
    return image(element.props.src, parseStyle(element.props.style));
  }

  const children = await collectChildren(element);
  const style = extractStyleFromProps(element.props);

  return container({
    children,
    ...parseStyle(style),
  });
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

async function collectChildren(
  element: ReactElement,
): Promise<Node[] | undefined> {
  if (
    typeof element.props !== "object" ||
    element.props === null ||
    !("children" in element.props)
  )
    return;

  const result = await fromJsx(element.props.children as ReactNode);
  if (!result) return;

  return Array.isArray(result) ? flattenNodes(result) : [result];
}

async function collectIterable(iterable: Iterable<ReactNode>): Promise<Node[]> {
  const children = await Promise.all(Array.from(iterable).map(fromJsx));

  return flattenNodes(children.filter((x): x is Nodes => x !== undefined));
}
