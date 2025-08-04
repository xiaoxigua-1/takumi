async function createObjectURL(blob: Blob) {
  return `data:${blob.type};base64,${btoa(
    (await blob.bytes()).reduce(
      (data, byte) => data + String.fromCharCode(byte),
      "",
    ),
  )}`;
}

export async function fetchLogo() {
  const response = await fetch("https://yeecord.com/img/logo.png");
  const blob = await response.blob();

  return createObjectURL(blob);
}
