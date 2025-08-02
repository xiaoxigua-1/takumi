import { describe, expect, test } from "bun:test";
import type { Gradient } from "../../src";
import { parseLinearGradient } from "../../src/jsx/linear-gradient-parsing";

describe("parseLinearGradient()", () => {
  test("parses angle in radians and normalizes to degrees", () => {
    const gradient = parseLinearGradient(
      `linear-gradient(${Math.PI / 2}rad, #000 0%, #fff 100%)`,
    );

    expect(gradient).toEqual({
      angle: 90,
      stops: [
        { color: 0x000000, position: 0 },
        { color: 0xffffff, position: 1 },
      ],
    } satisfies Gradient);
  });

  test("parses angle in turns and normalizes to degrees", () => {
    const gradient = parseLinearGradient(
      "linear-gradient(0.25turn, #000 0%, #fff 100%)",
    );

    expect(gradient).toEqual({
      angle: 90,
      stops: [
        { color: 0x000000, position: 0 },
        { color: 0xffffff, position: 1 },
      ],
    } satisfies Gradient);
  });

  test("parses diagonal direction 'to top right'", () => {
    const gradient = parseLinearGradient(
      "linear-gradient(to top right, #111 0%, #eee 100%)",
    );

    expect(gradient).toEqual({
      angle: 45,
      stops: [
        { color: 0x111111, position: 0 },
        { color: 0xeeeeee, position: 1 },
      ],
    } satisfies Gradient);
  });

  test("distributes positions evenly when not provided", () => {
    const gradient = parseLinearGradient(
      "linear-gradient(to bottom, #111, #222, #333, #444)",
    );

    expect(gradient).toEqual({
      angle: 180,
      stops: [
        { color: 0x111111, position: 0 },
        { color: 0x222222, position: 1 / 3 },
        { color: 0x333333, position: 2 / 3 },
        { color: 0x444444, position: 1 },
      ],
    } satisfies Gradient);
  });

  test("interpolates missing positions between known neighbors", () => {
    const gradient = parseLinearGradient(
      "linear-gradient(180deg, #000 0%, #777, #ddd 100%)",
    );
    expect(gradient).toEqual({
      angle: 180,
      stops: [
        { color: 0x000000, position: 0 },
        { color: 0xdddddd, position: 1 },
      ],
    } satisfies Gradient);
  });

  test("supports rgb() stop with numeric unitless position (0..1)", () => {
    const gradient = parseLinearGradient(
      "linear-gradient(90deg, rgb(255,0,0) 0, rgb(0,0,255) 1)",
    );

    expect(gradient).toEqual({
      angle: 90,
      stops: [
        { color: 0xff0000, position: 0 },
        { color: 0x0000ff, position: 1 },
      ],
    } satisfies Gradient);
  });

  test("clamps positions to [0,1] and sorts by position", () => {
    const gradient = parseLinearGradient(
      "linear-gradient(90deg, #000 -10%, #333 50%, #fff 120%)",
    );

    expect(gradient).toEqual({
      angle: 90,
      stops: [
        { color: 0x000000, position: 0 },
        { color: 0x333333, position: 0.5 },
        { color: 0xffffff, position: 1 },
      ],
    } satisfies Gradient);
  });

  test("normalizes negative angles and angles >= 360 to [0,360)", () => {
    const g1 = parseLinearGradient(
      "linear-gradient(-90deg, #000 0%, #fff 100%)",
    );
    const g2 = parseLinearGradient(
      "linear-gradient(450deg, #000 0%, #fff 100%)",
    );

    expect(g1).toEqual({
      angle: 270,
      stops: [
        { color: 0x000000, position: 0 },
        { color: 0xffffff, position: 1 },
      ],
    } satisfies Gradient);
    expect(g2).toEqual({
      angle: 90,
      stops: [
        { color: 0x000000, position: 0 },
        { color: 0xffffff, position: 1 },
      ],
    } satisfies Gradient);
  });

  test("throws for insufficient color stops", () => {
    expect(() =>
      parseLinearGradient("linear-gradient(to right, #000)"),
    ).toThrow();
  });
});
