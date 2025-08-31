import type { ComponentProps, JSX, ReactElement } from "react";

export function isHtmlElement<T extends keyof JSX.IntrinsicElements>(
  element: ReactElement,
  type: T,
): element is ReactElement<ComponentProps<T>> {
  return element.type === type;
}

export function isReactElement(element: unknown): element is ReactElement {
  return (
    typeof element === "object" &&
    element !== null &&
    "type" in element &&
    typeof element.type === "string"
  );
}
