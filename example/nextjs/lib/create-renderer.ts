import { readFile } from "node:fs/promises";
import { Renderer } from "@takumi-rs/core";
import { container, percentage, rem, rgba, text } from "@takumi-rs/helpers";

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
    style: {
      width: percentage(100),
      height: percentage(100),
      flexDirection: "column",
      backgroundImage: [
        {
          angle: 35,
          stops: [
            { color: rgba(246, 249, 255, 1), hint: 0 }, // near-white with cool tint
            { color: rgba(236, 243, 252, 1), hint: 0.22 }, // soft blue-gray
            { color: rgba(228, 238, 250, 1), hint: 0.55 }, // gentle body tone
            { color: rgba(243, 247, 255, 1), hint: 0.78 }, // lift
            { color: rgba(250, 252, 255, 1), hint: 1 }, // almost white
          ],
        },
      ],
      boxShadow: [
        {
          offsetX: 0,
          offsetY: 0,
          blurRadius: 16,
          spreadRadius: 2,
          color: rgba(0, 0, 0, 0.1),
          inset: true,
        },
      ],
      backgroundColor: rgba(255, 255, 255, 0.18),
      gap: rem(2),
      padding: [0, rem(12)],
      justifyContent: "center",
    },
    children: [
      text(`Hello, ${name}!`, {
        fontSize: 64,
        fontWeight: 600,
        color: {
          angle: 45,
          stops: [
            { color: 0xff0f7b, hint: 0 }, // dark text
            { color: 0xf89b29, hint: 1 }, // lighter text
          ],
        },
      }),
      text("This is a component created with Takumi!", {
        fontSize: 36,
      }),
      container({
        style: {
          width: percentage(100),
          height: 1,
          backgroundColor: rgba(0, 0, 0, 0.1),
        },
      }),
      text(
        `Server time right now is ${intl.format(new Date())} (${Date.now()})`,
        {
          color: rgba(0, 0, 0, 0.75),
          fontSize: 24,
        },
      ),
    ],
  });
}
