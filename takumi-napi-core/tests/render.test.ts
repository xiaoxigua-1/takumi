import { test, expect } from "bun:test";
import { Renderer } from "../index";
import { container, image } from "@takumi/helpers";

test("render", async () => {
  const render = new Renderer({
    fonts: [],
  });

  const result = render.render(
    container({
      children: [
        image("https://yeecord.com/img/logo.png", {
          width: 128,
          height: 128,
        }),
      ],
      background_color: 0xffffff,
      width: {
        percentage: 100,
      },
      height: {
        percentage: 100,
      },
    })
  );

  expect(result).toBeInstanceOf(Buffer);
});
