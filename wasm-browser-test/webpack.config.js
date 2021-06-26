const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");
module.exports = {
  mode: "development",
  entry: "./src/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async",
      },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [new HtmlWebpackPlugin({ title: "Mocha" })],
  devServer: {
    contentBase: path.join(__dirname, "dist"),
  },
};
