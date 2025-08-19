import { Box, Brain, Globe, type LucideIcon, Zap } from "lucide-react";
import { createElement } from "react";

const secondaryForeground = "rgba(241, 245, 249, 0.9)";
const primaryForeground = "#F1F5F9";
const borderColor = "rgba(215, 29, 54, 0.5)";
const accentColor = "#ff3535";

export function Component() {
  return (
    <div
      style={{
        backgroundImage:
          "linear-gradient(135deg, #100806 0%, #1b0a08 35%, #2a0c0a 65%, #360e0c 100%)",
        width: "100%",
        height: "100%",
        fontFamily: "Plus Jakarta Sans",
        display: "grid",
        gridTemplateColumns: "3fr 4fr",
        borderColor,
      }}
    >
      <div
        style={{
          padding: "4rem",
          display: "flex",
          flexDirection: "column",
          borderRightWidth: 1,
          borderColor,
          justifyContent: "center",
          color: primaryForeground,
          gap: "2rem",
        }}
      >
        <img
          src="takumi.svg"
          alt="Takumi logo"
          style={{
            width: "6rem",
          }}
        />
        <span style={{ fontSize: "4rem", fontWeight: 800 }}>Takumi</span>
        <span
          style={{
            fontSize: "2rem",
            fontWeight: 600,
            color: secondaryForeground,
            lineHeight: 1.5,
          }}
        >
          Render your React components to images.
        </span>
      </div>

      <div style={{ display: "flex", flexDirection: "column" }}>
        <div
          style={{
            justifyContent: "center",
            alignItems: "center",
            padding: "1rem",
            borderBottomWidth: 1,
            borderColor,
          }}
        >
          <span
            style={{
              color: accentColor,
              fontWeight: 800,
              fontSize: "1.5rem",
              padding: "0.5rem 1rem",
            }}
          >
            Build for Developers.
          </span>
        </div>
        <Features />
        <div
          style={{
            alignItems: "center",
            justifyContent: "center",
            padding: "0.75rem",
          }}
        >
          <span
            style={{
              color: secondaryForeground,
              fontWeight: 600,
              fontSize: "1rem",
            }}
          >
            This image was rendered with Takumi.
          </span>
        </div>
      </div>
    </div>
  );
}

function Features() {
  return (
    <div
      style={{
        display: "grid",
        gridTemplateColumns: "repeat(2, 1fr)",
        gridTemplateRows: "repeat(2, 1fr)",
        flexGrow: 1,
      }}
    >
      <Feature
        title="All in One"
        description="React code to image file in a single library."
        icon={Box}
        borderBottom
        borderRight
      />
      <Feature
        title="Speed is the Priority"
        description="Focus on speed to get your images ready."
        icon={Zap}
        borderBottom
      />
      <Feature
        title="Runs Everywhere"
        description="In the browser, Node.js, and Edge Runtime."
        icon={Globe}
        borderRight
        borderBottom
      />
      <Feature
        title="LLM Friendly"
        description="Documentation is ready for AI to use."
        icon={Brain}
        borderBottom
      />
    </div>
  );
}

function Feature({
  title,
  description,
  icon,
  borderBottom = false,
  borderRight = false,
}: {
  title: string;
  description: string;
  icon: LucideIcon;
  borderBottom?: boolean;
  borderRight?: boolean;
}) {
  return (
    <div
      style={{
        flexDirection: "column",
        borderBottomWidth: Number(borderBottom),
        borderRightWidth: Number(borderRight),
        borderColor,
        padding: "2rem",
        gap: "1rem",
        justifyContent: "center",
      }}
    >
      <div
        style={{
          gap: "0.5rem",
          color: secondaryForeground,
          alignItems: "center",
        }}
      >
        {createElement(icon, {
          color: accentColor,
          strokeWidth: 2.5,
          width: 24,
          height: 24,
        })}
        <span style={{ fontSize: "1.25rem", fontWeight: 600 }}>{title}</span>
      </div>
      <span
        style={{
          fontSize: "1.5rem",
          color: primaryForeground,
          fontWeight: 600,
          lineHeight: 1.5,
        }}
      >
        {description}
      </span>
    </div>
  );
}
