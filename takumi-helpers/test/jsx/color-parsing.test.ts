import { describe, expect, test } from "bun:test";
import { parseColor } from "../../src/jsx/color-parsing";

describe("color-parsing", () => {
  test("parses hex colors correctly", () => {
    expect(parseColor("#ff0000")).toBe(0xff0000);
    expect(parseColor("#00ff00")).toBe(0x00ff00);
    expect(parseColor("#0000ff")).toBe(0x0000ff);
    expect(parseColor("#ffffff")).toBe(0xffffff);
    expect(parseColor("#000000")).toBe(0x000000);
  });

  test("parses hex colors with alpha correctly", () => {
    expect(parseColor("#ff0000ff")).toEqual([255, 0, 0, 1]);
    expect(parseColor("#00ff0080")).toEqual([0, 255, 0, 0.5019607843137255]);
    expect(parseColor("#0000ff00")).toEqual([0, 0, 255, 0]);
    expect(parseColor("#ff00")).toEqual([255, 255, 0, 0]);
  });

  test("parses rgb colors correctly", () => {
    expect(parseColor("rgb(255, 0, 0)")).toEqual(0xff0000);
    expect(parseColor("rgb(0, 255, 0)")).toEqual(0x00ff00);
    expect(parseColor("rgb(0, 0, 255)")).toEqual(0x0000ff);
    expect(parseColor("rgb(128, 128, 128)")).toEqual(0x808080);
  });

  test("throws error for invalid hex colors", () => {
    expect(() => parseColor("#ff")).toThrow("Invalid hex color: #ff");
    expect(() => parseColor("#ff000")).toThrow("Invalid hex color: #ff000");
    expect(() => parseColor("#ff0000000")).toThrow(
      "Invalid hex color: #ff0000000",
    );
  });

  test("parses rgba colors correctly", () => {
    expect(parseColor("rgba(255, 0, 0, 0.5)")).toEqual([255, 0, 0, 0.5]);
    expect(parseColor("rgba(0, 255, 0, 1)")).toEqual([0, 255, 0, 1]);
    expect(parseColor("rgba(0, 0, 255, 0)")).toEqual([0, 0, 255, 0]);
    expect(parseColor("rgba(128, 128, 128, 0.3)")).toEqual([
      128, 128, 128, 0.3,
    ]);
    expect(parseColor("rgba(255, 255, 255, 0.75)")).toEqual([
      255, 255, 255, 0.75,
    ]);
    expect(parseColor("rgba(0, 0, 0, 0.99)")).toEqual([0, 0, 0, 0.99]);
  });

  test("parses rgba colors with spaces correctly", () => {
    expect(parseColor("rgba( 255 , 0 , 0 , 0.5 )")).toEqual([255, 0, 0, 0.5]);
    expect(parseColor("rgba(255,0,0,1)")).toEqual([255, 0, 0, 1]);
    expect(parseColor("rgba(  0  ,  255  ,  0  ,  0.25  )")).toEqual([
      0, 255, 0, 0.25,
    ]);
  });

  test("throws error for invalid rgb colors", () => {
    expect(() => parseColor("rgb(255, 0)")).toThrow(
      "Invalid rgb color: rgb(255, 0)",
    );
    expect(() => parseColor("rgb(255, 0, 0, 0.5)")).toThrow(
      "Invalid rgb color: rgb(255, 0, 0, 0.5)",
    );
    expect(() => parseColor("rgb(255, 0, 0, 0.5, 0.2)")).toThrow(
      "Invalid rgb color: rgb(255, 0, 0, 0.5, 0.2)",
    );
  });

  test("throws error for invalid rgba colors", () => {
    expect(() => parseColor("rgba(255, 0)")).toThrow(
      "Invalid rgba color: rgba(255, 0)",
    );
    expect(() => parseColor("rgba(255, 0, 0)")).toThrow(
      "Invalid rgba color: rgba(255, 0, 0)",
    );
    expect(() => parseColor("rgba(255, 0, 0, 0.5, 0.2)")).toThrow(
      "Invalid rgba color: rgba(255, 0, 0, 0.5, 0.2)",
    );
    expect(() => parseColor("rgba(255, 0, 0, 1.5)")).toThrow(
      "Invalid rgba color: rgba(255, 0, 0, 1.5)",
    );
    expect(() => parseColor("rgba(255, 0, 0, -0.1)")).toThrow(
      "Invalid rgba color: rgba(255, 0, 0, -0.1)",
    );
    expect(() => parseColor("rgba(256, 0, 0, 0.5)")).toThrow(
      "Invalid rgba color: rgba(256, 0, 0, 0.5)",
    );
    expect(() => parseColor("rgba(-1, 0, 0, 0.5)")).toThrow(
      "Invalid rgba color: rgba(-1, 0, 0, 0.5)",
    );
  });

  test("throws error for unsupported color formats", () => {
    expect(() => parseColor("red")).toThrow("Invalid color: red");
    expect(() => parseColor("hsl(0, 100%, 50%)")).toThrow(
      "Invalid color: hsl(0, 100%, 50%)",
    );
    expect(() => parseColor("linear-gradient(red, blue)")).toThrow(
      "Invalid color: linear-gradient(red, blue)",
    );
  });
});
