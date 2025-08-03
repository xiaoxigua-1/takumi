import { Renderer } from "@takumi-rs/core";
import { container, percentage, rem, rgba, text } from "@takumi-rs/helpers";
import { readFile } from "fs/promises";

const regular = await readFile(
  "../../assets/fonts/noto-sans/NotoSans-Regular.ttf",
);
const medium = await readFile(
  "../../assets/fonts/noto-sans/NotoSans-Medium.ttf",
);

export function createRenderer() {
  return new Renderer({
    fonts: [regular.buffer as ArrayBuffer, medium.buffer as ArrayBuffer],
  });
}

export function createComponent(name: string) {
  const intl = new Intl.DateTimeFormat("en-US", {
    timeStyle: "long",
  });

  return container({
    width: percentage(100),
    height: percentage(100),
    flex_direction: "column",
    background_color: 0xffffff,
    gap: rem(2),
    padding: [0, rem(12)],
    justify_content: "center",
    children: [
      text(`Hello, ${name}!`, {
        font_size: 64,
        font_weight: 600,
      }),
      text("This is a component created with Takumi!", {
        font_size: 36,
      }),
      container({
        width: percentage(100),
        height: 1,
        background_color: rgba(0, 0, 0, 0.1),
      }),
      text(
        `Server time right now is ${intl.format(new Date())} (${Date.now()})`,
        {
          color: rgba(0, 0, 0, 0.75),
          font_size: 24,
        },
      ),
    ],
  });
}
