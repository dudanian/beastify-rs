const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: {
    bootstrap: "./js/bootstrap.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        path.resolve(__dirname, "static")
      ]
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
      outName: "choose_beast_rs",
    }),
  ]
};
