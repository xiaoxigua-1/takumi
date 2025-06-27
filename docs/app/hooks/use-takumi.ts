import type * as takumi from "@takumi-rs/wasm";
import { useEffect, useState } from "react";

export function useTakumi() {
  const [module, setModule] = useState<typeof takumi>();

  useEffect(() => {
    void import("@takumi-rs/wasm").then(setModule);
  }, []);

  return module;
}
