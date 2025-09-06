import { writeFile } from "node:fs/promises";
import { fromJsx } from "@takumi-rs/helpers/jsx";
import { bench, run, summary } from "mitata";
import { Renderer } from "../..";

function createNode() {
  return fromJsx({
    type: "div",
    props: {
      style: {
        width: "1200px",
        height: "630px",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        fontSize: "100px",
        backgroundColor: "#fff",
      },
      children: [
        {
          type: "div",
          props: {
            style: {
              display: "flex",
              flexDirection: "column",
              alignItems: "flex-start",
              gap: 20,
              padding: 20,
            },
            children: [
              {
                type: "span",
                props: {
                  style: {
                    color: "#000",
                    fontSize: "40px",
                    fontWeight: "bold",
                  },
                  children: "Next.js Quickstart",
                },
              },
              {
                type: "span",
                props: {
                  style: {
                    color: "#000",
                    fontSize: "26px",
                    lineHeight: "1.4",
                  },
                  children:
                    "Learn how to integrate LIB_NAME into your Next.js application with this step-by-step guide. Well cover installation, configuration, and basic usage.",
                },
              },
              {
                type: "span",
                props: {
                  style: {
                    color: "#444",
                    fontSize: "20px",
                    padding: "20px",
                    marginTop: "20px",
                    borderWidth: "2px",
                    borderColor: "#888",
                    borderRadius: "10px",
                  },
                  children: "npx LIB_NAME",
                },
              },
            ],
          },
        },
        {
          type: "div",
          props: {
            style: {
              display: "flex",
              flex: "1",
              height: "100%",
              alignItems: "center",
              justifyContent: "center",
              backgroundImage:
                "linear-gradient(to bottom, lightgray 1px, transparent 1px), linear-gradient(to right, lightgray 1px, transparent 1px)",
              backgroundSize: "100px 100px",
            },
            children: [
              {
                type: "div",
                props: {
                  style: {
                    display: "flex",
                    flexDirection: "column",
                    flex: "1",
                    borderWidth: "2px",
                    borderColor: "#ccc",
                    borderRadius: "10px",
                    padding: "20px",
                    backgroundColor: "#fff",
                    transform: "translateX(-100px)",
                  },
                  children: [
                    {
                      type: "span",
                      props: {
                        style: {
                          color: "#333",
                          fontSize: "24px",
                          fontWeight: "600",
                          marginBottom: "10px",
                        },
                        children: "We value your privacy",
                      },
                    },
                    {
                      type: "span",
                      props: {
                        style: {
                          color: "#333",
                          fontSize: "16px",
                        },
                        children:
                          "This site uses cookies to improve your browsing experience, analyze site traffic, and show personalized content.",
                      },
                    },
                    {
                      type: "div",
                      props: {
                        style: {
                          display: "flex",
                          justifyContent: "flex-start",
                          gap: "10px",
                          marginTop: "20px",
                          paddingTop: "20px",
                          borderTopWidth: "1px",
                          borderTopColor: "#ccc",
                          width: "100%",
                        },
                        children: [
                          {
                            type: "button",
                            props: {
                              style: {
                                padding: "10px 20px",
                                fontSize: "16px",
                                color: "#777",
                                borderWidth: "1px",
                                borderColor: "#aaa",
                                borderRadius: "5px",
                              },
                              children: "Reject All",
                            },
                          },
                          {
                            type: "button",
                            props: {
                              style: {
                                padding: "10px 20px",
                                fontSize: "16px",
                                color: "#777",
                                borderWidth: "1px",
                                borderColor: "#aaa",
                                borderRadius: "5px",
                              },
                              children: "Accept All",
                            },
                          },
                          {
                            type: "div",
                            props: {
                              style: {
                                flex: "1",
                              },
                            },
                          },
                          {
                            type: "button",
                            props: {
                              style: {
                                padding: "10px 20px",
                                fontSize: "16px",
                                color: "#66d1bd",
                                borderWidth: "1px",
                                borderColor: "#66d1bd",
                                borderRadius: "5px",
                              },
                              children: "Customize",
                            },
                          },
                        ],
                      },
                    },
                  ],
                },
              },
            ],
          },
        },
      ],
    },
  });
}

const renderer = new Renderer();

summary(() => {
  bench("createNode", createNode);

  bench("createNode + renderAsync", async () => {
    const node = await createNode();
    return renderer.renderAsync(node, {
      width: 1200,
      height: 630,
    });
  });
});

if (process.argv.includes("--write")) {
  await writeFile(
    "output.png",
    renderer.render(await createNode(), {
      width: 1200,
      height: 630,
    }),
  );
}

await run();
