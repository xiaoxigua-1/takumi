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

type StripPrefix<T> = T extends `${(typeof prefixes)[number]}${infer Rest}`
  ? Rest
  : T;

export type RemoveGlobalsAndPrefixed<T> = Exclude<StripPrefix<T>, Globals>;

export function removeGlobalValues<T>(
  source: T,
): RemoveGlobalsAndPrefixed<T> | undefined {
  if (typeof source !== "string") {
    return source as RemoveGlobalsAndPrefixed<T>;
  }

  if (isGlobalValue(source)) {
    return;
  }

  if (source[0] === "-") {
    for (const prefix of prefixes) {
      if (source.startsWith(prefix)) {
        return source.slice(prefix.length) as RemoveGlobalsAndPrefixed<T>;
      }
    }

    throw new Error(`Unsupported prefix in value: ${source}`);
  }

  return source as RemoveGlobalsAndPrefixed<T>;
}

export function isGlobalValue(value: string): value is Globals {
  return (
    value === "inherit" ||
    value === "initial" ||
    value === "revert" ||
    value === "unset"
  );
}
