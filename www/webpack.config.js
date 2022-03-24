const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  experiments: {
    syncWebAssembly: true,
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/sync",
      },
    ],
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: ["index.html"],
    }),
  ],
};
