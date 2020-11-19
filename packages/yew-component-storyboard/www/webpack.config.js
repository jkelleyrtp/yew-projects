const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

// Create a config for the main app
// We don't need anything terribly fancy
// The HtmlWebpackPlugin is not important, but provides an html
// template to inject some scripts into before mounting Yew.
const appConfig = {
  entry: "./app/main.js",
  devServer: {
    contentBase: dist,
  },
  plugins: [
    // This just gives us a basic html file to start from
    new HtmlWebpackPlugin({
      template: "index.html",
    }),

    // This compiles our crate and generats the lib
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../crate-wasm"),
    }),
  ],
  resolve: {
    extensions: [".js"],
  },
  output: {
    path: dist,
    filename: "app.js",
  },
};

// This config actually generates both
const workerConfig = {
  entry: "./worker/worker.js",
  target: "webworker",
  plugins: [],
  resolve: {
    extensions: [".js", ".wasm"],
  },
  output: {
    path: dist,
    filename: "worker.js",
  },
};

module.exports = [appConfig, workerConfig];
