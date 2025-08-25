import { join } from "node:path";
import { file } from "bun";

export const persistentImages = [
  {
    src: "takumi.svg",
    data: await file("../../assets/images/takumi.svg").arrayBuffer(),
  },
];

const weights = ["600", "800"];

export const name = "500-stars";

export const width = 1200;
export const height = 675;

export const fonts = weights.map((weight) =>
  join(
    "../../assets/fonts/plus-jakarta-sans-v11-latin",
    `plus-jakarta-sans-v11-latin-${weight}.woff2`,
  ),
);

export default function FiveHundredStars() {
  return (
    <div
      style={{
        backgroundImage:
          "radial-gradient(circle at center bottom, rgba(227, 179, 65, 1.0), rgba(227, 179, 65, 0.0) 75%)",
        backgroundColor: "black",
        width: "100%",
        height: "100%",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <img
        src={persistentImages[0]?.src}
        alt="500 Stars"
        style={{
          width: "12rem",
          aspectRatio: 1,
          marginBottom: "2rem",
        }}
      />
      <p
        style={{
          color: "rgba(255, 255, 255, 0.85)",
          fontSize: "8rem",
          fontWeight: 800,
        }}
      >
        500 Stars
      </p>
      <p
        style={{
          color: "rgba(255, 255, 255, 0.75)",
          fontSize: "2rem",
          fontWeight: 600,
        }}
      >
        Background noise will be in the next version of Takumi.
      </p>
      <div
        style={{
          position: "absolute",
          top: 0,
          left: 0,
          width: "100%",
          height: "100%",
          backgroundImage: "noise-v1(opacity(0.3) frequency(1.0) octaves(4))",
        }}
      />
    </div>
  );
}
