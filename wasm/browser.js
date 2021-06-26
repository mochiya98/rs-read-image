import { readImage as wasmReadImage } from "./dist/browser/read_image_wasm.js";

export function readImage(buf) {
  // 引数0の時に分かりづらいエラーが出るのでその対応
  if (!buf) throw new Error("invalid_argument");
  return wasmReadImage(buf);
}
