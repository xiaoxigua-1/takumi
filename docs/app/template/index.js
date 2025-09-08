import { container, percentage } from "@takumi-rs/helpers";
import { renderer } from "./shared/renderer";

const root = container({
  style: {
    backgroundColor: 0xff0000,
    width: percentage(100),
    height: percentage(100),
  },
});

const image = renderer.renderAsDataUrl(root, 1200, 630, "webp");

const app = document.getElementById("app");

app.innerHTML = `<img src="${image}"/>`;
