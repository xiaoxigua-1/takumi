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
    width: percentage(100),
    height: percentage(100),
    flex_direction: "column",
    background_image: {
      angle: 35,
      stops: [
        { color: rgba(246, 249, 255, 1), position: 0 }, // near-white with cool tint
        { color: rgba(236, 243, 252, 1), position: 0.22 }, // soft blue-gray
        { color: rgba(228, 238, 250, 1), position: 0.55 }, // gentle body tone
        { color: rgba(243, 247, 255, 1), position: 0.78 }, // lift
        { color: rgba(250, 252, 255, 1), position: 1 }, // almost white
      ],
    },
    box_shadow: [
      {
        offset_x: 0,
        offset_y: 0,
        blur_radius: 16,
        spread_radius: 2,
        color: rgba(0, 0, 0, 0.1),
        inset: true,
      },
    ],
    background_color: rgba(255, 255, 255, 0.18),

    gap: rem(2),
    padding: [0, rem(12)],
    justify_content: "center",
    children: [
      text(`Hello, ${name}!`, {
        font_size: 64,
        font_weight: 600,
        color: {
          angle: 45,
          stops: [
            { color: 0xff0f7b, position: 0 }, // dark text
            { color: 0xf89b29, position: 1 }, // lighter text
          ],
        },
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
