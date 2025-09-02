import type { ComponentProps, JSX, ReactNode } from "react";
import type { ReactElementLike } from "./jsx";

export function isHtmlElement<T extends keyof JSX.IntrinsicElements>(
  element: ReactElementLike,
  type: T,
): element is ReactElementLike<T, ComponentProps<T>> {
  return element.type === type;
}

export function isReactElement(element: unknown): element is ReactElementLike {
  return (
    typeof element === "object" &&
    element !== null &&
    "type" in element &&
    (typeof element.type === "string" ||
      typeof element.type === "function" ||
      typeof element.type === "symbol" ||
      isReactForwardRef(element.type))
  );
}

export function camelToKebab(camel: string): string {
  return camel.replace(/([A-Z])/g, "-$1").toLowerCase();
}

export function isValidElement(object: unknown): object is ReactElementLike {
  return (
    typeof object === "object" &&
    object !== null &&
    "type" in object &&
    (typeof object.type === "string" ||
      typeof object.type === "symbol" ||
      isFunctionComponent(object.type) ||
      isReactForwardRef(object.type))
  );
}

export function isFunctionComponent(
  value: unknown,
): value is (props: unknown) => ReactNode {
  return typeof value === "function";
}

export const REACT_FORWARD_REF_TYPE = Symbol.for("react.forward_ref");
export const REACT_MEMO_TYPE = Symbol.for("react.memo");

export function isReactForwardRef(type: unknown): boolean {
  return (
    typeof type === "object" &&
    type !== null &&
    "$$typeof" in type &&
    type.$$typeof === REACT_FORWARD_REF_TYPE
  );
}

export function isReactMemo(type: unknown): boolean {
  return (
    typeof type === "object" &&
    type !== null &&
    "$$typeof" in type &&
    type.$$typeof === REACT_MEMO_TYPE
  );
}
