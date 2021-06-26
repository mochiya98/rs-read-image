//import { readImage } from "../wasm/pkg/rs_read_image_wasm";
import { readImage } from "../..";
import { expect } from "chai";
import { runMocha } from "./mocha";
window.readImage = readImage;

runMocha(() => {
  describe("readImage", () => {
    const dummyPng = Uint8Array.from(
      atob(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR4AWPgUbLwAwABWQC1tLup9QAAAABJRU5ErkJggg=="
      ),
      (c) => c.charCodeAt(0)
    );

    it("successful", () => {
      const result = readImage(dummyPng);
      expect(Array.from(result)).to.eql([12, 34, 56, 78]);
    });
    it("failed", () => {
      expect(() => readImage([])).to.throws(
        "The image format could not be determined"
      );
    });
  });
});
