import { describe, expect, test } from "bun:test";
import { em, percentage, rem, vh } from "../../src/helpers";
import { fromJsx } from "../../src/jsx/jsx";
import type { ContainerNode, ImageNode } from "../../src/types";

describe("style-parser", () => {
  test("handles dimensions in style props", async () => {
    const result = await fromJsx(
      <div style={{ width: 100, height: "50px", maxWidth: "100%" }}>Test</div>,
    );

    expect(result[0]).toEqual({
      type: "container",
      width: 100,
      height: 50,
      max_width: percentage(100),
      children: [{ type: "text", text: "Test" }],
    } satisfies ContainerNode);
  });

  test("handles spacing in style props", async () => {
    const result = await fromJsx(
      <div style={{ padding: "10px 20px", margin: 15 }}>Test</div>,
    );

    expect(result[0]).toEqual({
      type: "container",
      padding: [10, 20],
      margin: 15,
      children: [{ type: "text", text: "Test" }],
    } satisfies ContainerNode);
  });

  test("handles flex properties in style props", async () => {
    const result = await fromJsx(
      <div
        style={{
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "flex-start",
          flexGrow: 1,
          gap: "8px",
        }}
      >
        Test
      </div>,
    );

    expect(result[0]).toEqual({
      type: "container",
      flex_direction: "column",
      justify_content: "center",
      align_items: "flex-start",
      flex_grow: 1,
      gap: 8,
      children: [{ type: "text", text: "Test" }],
    } satisfies ContainerNode);
  });

  test("handles text properties in style props", async () => {
    const result = await fromJsx(
      <div
        style={{
          fontSize: "16px",
          fontFamily: "Arial",
          fontWeight: "bold",
          lineHeight: 1.5,
          textAlign: "center",
        }}
      >
        Test
      </div>,
    );

    expect(result[0]).toEqual({
      type: "container",
      font_size: 16,
      font_family: "Arial",
      font_weight: 700,
      line_height: 1.5,
      text_align: "center",
      children: [{ type: "text", text: "Test" }],
    } satisfies ContainerNode);
  });

  test("handles different length units in style props", async () => {
    const result = await fromJsx(
      <div
        style={{
          width: "50%",
          height: "2rem",
          padding: "1em",
          margin: "10vh",
        }}
      >
        Test
      </div>,
    );

    expect(result[0]).toEqual({
      type: "container",
      width: percentage(50),
      height: rem(2),
      padding: em(1),
      margin: vh(10),
      children: [{ type: "text", text: "Test" }],
    } satisfies ContainerNode);
  });

  test("handles style props on img elements", async () => {
    const result = await fromJsx(
      <img
        src="https://example.com/image.jpg"
        alt="Test content"
        style={{ width: 200, height: 150 }}
      />,
    );

    expect(result[0]).toEqual({
      type: "image",
      src: "https://example.com/image.jpg",
      width: 200,
      height: 150,
    } satisfies ImageNode);
  });

  describe("colors", () => {
    test("handles background color in style props", async () => {
      const result = await fromJsx(
        <div style={{ backgroundColor: "#ff0000" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        background_color: 16711680,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles border color in style props", async () => {
      const result = await fromJsx(
        <div style={{ borderColor: "rgb(0, 255, 0)" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        border_color: 0x00ff00,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles text color in style props", async () => {
      const result = await fromJsx(
        <div style={{ color: "rgba(0, 0, 255, 0.5)" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        color: [0, 0, 255, 0.5],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });
  });

  describe("borders", () => {
    test("handles border width in style props", async () => {
      const result = await fromJsx(
        <div style={{ borderWidth: "2px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        border_width: 2,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles multiple border width values in style props", async () => {
      const result = await fromJsx(
        <div style={{ borderWidth: "1px 2px 3px 4px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        border_width: [1, 2, 3, 4],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles border radius in style props", async () => {
      const result = await fromJsx(
        <div style={{ borderRadius: "8px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        border_radius: 8,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles multiple border radius values in style props", async () => {
      const result = await fromJsx(
        <div style={{ borderRadius: "4px 8px 12px 16px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        border_radius: [4, 8, 12, 16],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });
  });

  describe("positioning", () => {
    test("handles absolute positioning in style props", async () => {
      const result = await fromJsx(
        <div style={{ position: "absolute" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        position: "absolute",
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles relative positioning in style props", async () => {
      const result = await fromJsx(
        <div style={{ position: "relative" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        position: "relative",
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles single inset value in style props", async () => {
      const result = await fromJsx(<div style={{ inset: "10px" }}>Test</div>);

      expect(result[0]).toEqual({
        type: "container",
        inset: 10,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles multiple inset values in style props", async () => {
      const result = await fromJsx(
        <div style={{ inset: "10px 20px 30px 40px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        inset: [10, 20, 30, 40],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("individual inset values has higher priority over global inset", async () => {
      const result = await fromJsx(
        <div style={{ inset: "10px", top: "20px", left: "30px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        inset: [20, 10, 10, 30],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("individual inset values has higher priority over global inset", async () => {
      const result = await fromJsx(
        <div
          style={{
            inset: "10px 20px 30px 40px",
            top: "50px",
            right: "60px",
            bottom: "70px",
            left: "80px",
          }}
        >
          Test
        </div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        inset: [50, 60, 70, 80],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });
  });

  describe("spacing priority", () => {
    test("paddingTop overrides general padding", async () => {
      const result = await fromJsx(
        <div style={{ padding: "10px", paddingTop: "20px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        padding: [20, 10, 10, 10],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("marginLeft overrides general margin", async () => {
      const result = await fromJsx(
        <div style={{ margin: "15px", marginLeft: "25px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        margin: [15, 15, 15, 25],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });
  });

  describe("grid system", () => {
    test("handles grid auto columns in style props", async () => {
      const result = await fromJsx(
        <div style={{ gridAutoColumns: "100px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        grid_auto_columns: [100],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles grid auto columns with fr units in style props", async () => {
      const result = await fromJsx(
        <div style={{ gridAutoColumns: "1fr" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        grid_auto_columns: [{ fr: 1 }],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles grid auto rows in style props", async () => {
      const result = await fromJsx(
        <div style={{ gridAutoRows: "50px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        grid_auto_rows: [50],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles grid auto flow in style props", async () => {
      const result = await fromJsx(
        <div style={{ gridAutoFlow: "column" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        grid_auto_flow: "column",
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles grid column with string values in style props", async () => {
      const result = await fromJsx(
        <div style={{ gridColumn: "1 / 3" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        grid_column: { start: 1, end: 3 },
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles grid column with numeric values in style props", async () => {
      const result = await fromJsx(<div style={{ gridColumn: 2 }}>Test</div>);

      expect(result[0]).toEqual({
        type: "container",
        grid_column: { start: 2, end: null },
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles grid row with string values in style props", async () => {
      const result = await fromJsx(
        <div style={{ gridRow: "2 / 4" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        grid_row: { start: 2, end: 4 },
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles grid template columns in style props", async () => {
      const result = await fromJsx(
        <div style={{ gridTemplateColumns: "100px 1fr" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        grid_template_columns: [{ single: 100 }, { single: { fr: 1 } }],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles grid template rows in style props", async () => {
      const result = await fromJsx(
        <div style={{ gridTemplateRows: "50px 2fr" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        grid_template_rows: [{ single: 50 }, { single: { fr: 2 } }],
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });
  });

  describe("typography", () => {
    test("handles line clamp in style props", async () => {
      const result = await fromJsx(<div style={{ lineClamp: 3 }}>Test</div>);

      expect(result[0]).toEqual({
        type: "container",
        line_clamp: 3,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles line clamp none in style props", async () => {
      const result = await fromJsx(
        <div style={{ lineClamp: "none" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        line_clamp: 0,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles text overflow in style props", async () => {
      const result = await fromJsx(
        <div style={{ textOverflow: "ellipsis" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        text_overflow: "ellipsis",
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles letter spacing in style props", async () => {
      const result = await fromJsx(
        <div style={{ letterSpacing: "0.1em" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        letter_spacing: { em: 0.1 },
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });
  });

  describe("flexbox", () => {
    test("handles flex basis in style props", async () => {
      const result = await fromJsx(
        <div style={{ flexBasis: "100px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        flex_basis: 100,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles flex shrink in style props", async () => {
      const result = await fromJsx(<div style={{ flexShrink: 0.5 }}>Test</div>);

      expect(result[0]).toEqual({
        type: "container",
        flex_shrink: 0.5,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles flex wrap in style props", async () => {
      const result = await fromJsx(
        <div style={{ flexWrap: "wrap" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        flex_wrap: "wrap",
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles align content in style props", async () => {
      const result = await fromJsx(
        <div style={{ alignContent: "space-between" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        align_content: "space-between",
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles justify items in style props", async () => {
      const result = await fromJsx(
        <div style={{ justifyItems: "center" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        justify_items: "center",
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });
  });

  describe("sizing", () => {
    test("handles min width in style props", async () => {
      const result = await fromJsx(
        <div style={{ minWidth: "100px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        min_width: 100,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles min height in style props", async () => {
      const result = await fromJsx(
        <div style={{ minHeight: "50px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        min_height: 50,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles max height in style props", async () => {
      const result = await fromJsx(
        <div style={{ maxHeight: "200px" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        max_height: 200,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles aspect ratio in style props", async () => {
      const result = await fromJsx(
        <div style={{ aspectRatio: 16 / 9 }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        aspect_ratio: 16 / 9,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });
  });

  describe("visual effects", () => {
    test("handles object fit in style props", async () => {
      const result = await fromJsx(
        <img
          src="https://example.com/image.jpg"
          alt="Test content"
          style={{ width: 200, height: 150, objectFit: "cover" }}
        />,
      );

      expect(result[0]).toEqual({
        type: "image",
        src: "https://example.com/image.jpg",
        width: 200,
        height: 150,
        object_fit: "cover",
      } satisfies ImageNode);
    });
  });

  describe("edge cases and error handling", () => {
    test("handles invalid color values gracefully in style props", async () => {
      const result = await fromJsx(
        <div style={{ backgroundColor: "invalid-color" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles invalid box shadow gracefully in style props", async () => {
      const result = await fromJsx(
        <div style={{ boxShadow: "invalid-shadow" }}>Test</div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles zero values correctly in style props", async () => {
      const result = await fromJsx(
        <div
          style={{
            borderWidth: 0,
            borderRadius: 0,
            flexShrink: 0,
            letterSpacing: 0,
          }}
        >
          Test
        </div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        border_width: 0,
        border_radius: 0,
        flex_shrink: 0,
        letter_spacing: 0,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });

    test("handles combined properties in style props", async () => {
      const result = await fromJsx(
        <div
          style={{
            backgroundColor: "#ffffff",
            borderColor: "#000000",
            color: "#333333",
            borderWidth: "1px",
            borderRadius: "4px",
            position: "relative",
            inset: "10px",
            flexBasis: "auto",
            flexShrink: 1,
            minWidth: "100px",
            maxHeight: "200px",
            aspectRatio: 1,
          }}
        >
          Test
        </div>,
      );

      expect(result[0]).toEqual({
        type: "container",
        background_color: 16777215,
        border_color: 0,
        color: 3355443,
        border_width: 1,
        border_radius: 4,
        position: "relative",
        inset: 10,
        flex_basis: "auto",
        flex_shrink: 1,
        min_width: 100,
        max_height: 200,
        aspect_ratio: 1,
        children: [{ type: "text", text: "Test" }],
      } satisfies ContainerNode);
    });
  });
});
