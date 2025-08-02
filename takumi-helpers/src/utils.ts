import type { Globals } from "csstype";

export type CamelToSnakeCase<S extends string> =
  S extends `${infer T}${infer U}`
    ? `${T extends Capitalize<T> ? "_" : ""}${Lowercase<T>}${CamelToSnakeCase<U>}`
    : S;

export type SnakeToCamelCase<S extends string> =
  S extends `${infer T}_${infer U}`
    ? `${T}${Capitalize<SnakeToCamelCase<U>>}`
    : S;

export function camelToSnakeCase(str: string) {
  return str.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
}

const prefixes = ["-moz-", "-webkit-", "-o-", "-ms-"] as const;

export type RemoveGlobalsAndPrefixed<T> = Exclude<
  T,
  Globals | `${(typeof prefixes)[number]}${string}`
>;

export function removeGlobalValues<T>(
  source: T,
): RemoveGlobalsAndPrefixed<T> | undefined {
  if (typeof source !== "string") {
    return source as RemoveGlobalsAndPrefixed<T>;
  }

  if (isGlobalValue(source) || isPrefixed(source)) {
    return;
  }

  return source as RemoveGlobalsAndPrefixed<T>;
}

function isPrefixed(value: string) {
  return prefixes.some((prefix) => value.startsWith(prefix));
}

export function isGlobalValue(value: string): value is Globals {
  return (
    value === "inherit" ||
    value === "initial" ||
    value === "revert" ||
    value === "unset"
  );
}
