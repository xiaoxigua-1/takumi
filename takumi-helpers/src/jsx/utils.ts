import type { ComponentProps, JSX } from "react";
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
    typeof element.type === "string"
  );
}

export function camelToKebab(camel: string): string {
  return camel.replace(/([A-Z])/g, "-$1").toLowerCase();
}
