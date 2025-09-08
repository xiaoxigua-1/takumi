import { describe, expect, test } from "bun:test";
import ImageResponse from "../src";

describe("ImageResponse", () => {
  test("should not crash", async () => {
    const response = new ImageResponse(<div>Hello</div>);

    expect(response.status).toBe(200);
    expect(response.headers.get("content-type")).toBe("image/webp");

    expect(await response.arrayBuffer()).toBeDefined();
  });

  test("should set content-type", async () => {
    const response = new ImageResponse(<div>Hello</div>, {
      width: 100,
      height: 100,
      format: "png",
    });

    expect(response.headers.get("content-type")).toBe("image/png");
    expect(await response.arrayBuffer()).toBeDefined();
  });
});
