export function Component() {
  return (
    <div
      style={{
        backgroundColor: "white",
        width: "100%",
        height: "100%",
        color: "#111",
        fontFamily: "Plus Jakarta Sans",
        padding: "64px 80px",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          height: "100%",
          gap: "24px",
          maxWidth: "980px",
          margin: "0 auto",
        }}
      >
        <div style={{ display: "flex", alignItems: "end", gap: "24px" }}>
          <img src="takumi.svg" alt="Takumi logo" />
          <span
            style={{
              fontSize: "96px",
              fontWeight: 700,
            }}
          >
            Takumi
          </span>
        </div>
        <p
          style={{
            fontSize: "32px",
            color: "#444",
            margin: 0,
            maxWidth: "960px",
          }}
        >
          High quality image rendering for Open Graph, server, and WASM.
        </p>
        <div
          style={{
            display: "flex",
            gap: "12px",
            marginTop: "8px",
          }}
        >
          <span
            style={{
              fontSize: "20px",
              color: "#666",
              backgroundColor: "#f3f4f6",
              padding: "8px 12px",
              borderRadius: "999px",
            }}
          >
            Rust core
          </span>
          <span
            style={{
              fontSize: "20px",
              color: "#666",
              backgroundColor: "#f3f4f6",
              padding: "8px 12px",
              borderRadius: "999px",
            }}
          >
            Node/WASM bindings
          </span>
        </div>
      </div>
    </div>
  );
}
