import { container, percentage, rem, rgba, text } from "@takumi-rs/helpers";

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
        maskImage: "linear-gradient(45deg, #ff0f7b 0%, #f89b29 100%)",
        color: 0,
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
