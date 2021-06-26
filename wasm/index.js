const { readImage } = require("./dist/node/read_image_wasm.js");
module.exports = {
  readImage(src) {
    if (!src) throw new Error("invalid_argument");
    return readImage(src);
  },
};
