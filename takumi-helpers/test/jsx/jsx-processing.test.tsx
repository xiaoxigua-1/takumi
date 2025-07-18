import { describe, expect, test } from "bun:test";
import { container } from "../../src/helpers";
import { fromJsx } from "../../src/jsx/jsx-processing";
import type { ContainerNode, ImageNode, Node, TextNode } from "../../src/types";

describe("fromJsx", () => {
  test("converts text to TextNode", async () => {
    const result = await fromJsx("Hello World");
    expect(result).toEqual([
      {
        type: "text",
        text: "Hello World",
      } satisfies TextNode,
    ]);
  });

  test("converts number to TextNode", async () => {
    const result = await fromJsx(42);
    expect(result).toEqual([
      {
        type: "text",
        text: "42",
      } satisfies TextNode,
    ]);
  });

  test("returns empty array for null/undefined", async () => {
    expect(await fromJsx(null)).toEqual([]);
    expect(await fromJsx(undefined)).toEqual([]);
  });

  test("converts simple div to ContainerNode", async () => {
    const result = await fromJsx(<div>Hello</div>);
    expect(result).toEqual([
      {
        type: "container",
        children: [
          {
            type: "text",
            text: "Hello",
          },
        ],
      } satisfies ContainerNode,
    ]);
  });

  test("handles function components", async () => {
    const MyComponent = ({ name }: { name: string }) => <div>Hello {name}</div>;

    const result = await fromJsx(<MyComponent name="World" />);
    expect(result).toEqual([
      {
        type: "container",
        children: [
          { type: "text", text: "Hello " },
          { type: "text", text: "World" },
        ],
      } satisfies ContainerNode,
    ]);
  });

  test("handles async function components", async () => {
    const AsyncComponent = async ({ name }: { name: string }) => (
      <div>Hello {name}</div>
    );

    const result = await fromJsx(<AsyncComponent name="Async" />);
    expect(result).toEqual([
      {
        type: "container",
        children: [
          { type: "text", text: "Hello " },
          { type: "text", text: "Async" },
        ],
      } satisfies ContainerNode,
    ]);
  });

  test("handles fragments", async () => {
    const result = await fromJsx(
      <>
        <div>First</div>
        <div>Second</div>
      </>,
    );

    expect(result).toEqual([
      {
        type: "container",
        children: [{ type: "text", text: "First" }],
      },
      {
        type: "container",
        children: [{ type: "text", text: "Second" }],
      },
    ] satisfies Node[]);
  });

  test("handles arrays", async () => {
    const items = ["First", "Second", "Third"];
    const result = await fromJsx(
      <div>
        {items.map((item) => (
          <span key={item}>{item}</span>
        ))}
      </div>,
    );

    expect(result).toEqual([
      {
        type: "container",
        children: [
          {
            type: "container",
            children: [{ type: "text", text: "First" }],
          },
          {
            type: "container",
            children: [{ type: "text", text: "Second" }],
          },
          {
            type: "container",
            children: [{ type: "text", text: "Third" }],
          },
        ],
      } satisfies ContainerNode,
    ]);
  });

  test("converts img elements to ImageNode", async () => {
    const result = await fromJsx(
      <img src="https://example.com/image.jpg" alt="Test" />,
    );
    expect(result).toEqual([
      {
        type: "image",
        src: "https://example.com/image.jpg",
      } satisfies ImageNode,
    ]);
  });

  test("handles img without src satisfies container", async () => {
    const result = await fromJsx(<img alt="No src" />);
    expect(result).toEqual([
      {
        type: "container",
        children: [],
      } satisfies ContainerNode,
    ]);
  });

  test("handles deeply nested structures", async () => {
    const result = await fromJsx(
      <div>
        <h1>Title</h1>
        <div>
          <p>
            Paragraph with <strong>bold</strong> text
          </p>
          <ul>
            <li>Item 1</li>
            <li>Item 2</li>
          </ul>
        </div>
      </div>,
    );

    expect(result).toEqual([
      {
        type: "container",
        children: [
          {
            type: "container",
            children: [{ type: "text", text: "Title" }],
          },
          {
            type: "container",
            children: [
              {
                type: "container",
                children: [
                  { type: "text", text: "Paragraph with " },
                  {
                    type: "container",
                    children: [{ type: "text", text: "bold" }],
                  },
                  { type: "text", text: " text" },
                ],
              },
              {
                type: "container",
                children: [
                  {
                    type: "container",
                    children: [{ type: "text", text: "Item 1" }],
                  },
                  {
                    type: "container",
                    children: [{ type: "text", text: "Item 2" }],
                  },
                ],
              },
            ],
          },
        ],
      } satisfies ContainerNode,
    ]);
  });

  test("handles promises", async () => {
    const promiseElement = Promise.resolve("Resolved text");
    const result = await fromJsx(promiseElement);
    expect(result).toEqual([
      {
        type: "text",
        text: "Resolved text",
      } satisfies TextNode,
    ]);
  });

  test("fromJsx result can be used as container() children", async () => {
    // Test single node result
    const jsxResult = await fromJsx(<div>Hello World</div>);
    const containerWithSingleChild = container({ children: jsxResult });

    expect(containerWithSingleChild).toEqual({
      type: "container",
      children: [
        {
          type: "container",
          children: [{ type: "text", text: "Hello World" }],
        },
      ],
    } satisfies ContainerNode);

    // Test array result (from fragments)
    const fragmentResult = await fromJsx(
      <>
        <div>First</div>
        <div>Second</div>
      </>,
    );
    const containerWithMultipleChildren = container({
      children: fragmentResult,
    });

    expect(containerWithMultipleChildren).toEqual({
      type: "container",
      children: [
        {
          type: "container",
          children: [{ type: "text", text: "First" }],
        },
        {
          type: "container",
          children: [{ type: "text", text: "Second" }],
        },
      ],
    } satisfies ContainerNode);

    // Test empty array for null/undefined
    const emptyResult = await fromJsx(null);
    const containerWithEmpty = container({ children: emptyResult });

    expect(containerWithEmpty).toEqual({
      type: "container",
      children: [],
    } satisfies ContainerNode);
  });

  test("integration: fromJsx result as container children with complex JSX", async () => {
    // Test complex JSX structure that can be directly used as container children
    const complexJsx = await fromJsx(
      <div>
        <h1>Welcome</h1>
        <div>
          <span>Item 1</span>
          <span>Item 2</span>
        </div>
        <img src="https://example.com/logo.png" alt="Logo" />
      </div>,
    );

    const complexContainer = container({
      children: complexJsx,
    });

    expect(complexContainer).toEqual({
      type: "container",
      children: [
        {
          type: "container",
          children: [
            {
              type: "container",
              children: [{ type: "text", text: "Welcome" }],
            },
            {
              type: "container",
              children: [
                {
                  type: "container",
                  children: [{ type: "text", text: "Item 1" }],
                },
                {
                  type: "container",
                  children: [{ type: "text", text: "Item 2" }],
                },
              ],
            },
            {
              type: "image",
              src: "https://example.com/logo.png",
            },
          ],
        },
      ],
    } satisfies ContainerNode);
  });
});
