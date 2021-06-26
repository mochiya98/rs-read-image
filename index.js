try {
  module.exports = require(`@mochiya98/rs-read-image-${process.platform}-${process.arch}`);
} catch (e) {
  module.exports = require(`./wasm/index.js`);
}
