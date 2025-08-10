const path = require("path")
const HtmlWebpackPlugin = require("html-webpack-plugin")
const { CleanWebpackPlugin } = require("clean-webpack-plugin")

module.exports = {
  mode: "none",
  entry: {
    index: "./index.js",
  },
  devServer: {
    static: {
      directory: path.join(__dirname, "dist"),
    },
    hot: true,
    port: 8080,
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        include: /pkg/,
        loader: "file-loader",
        type: "javascript/auto",
        sideEffects: true,
        options: {
          name: "[name].[ext]",
        },
      },
      {
        test: /\.css$/,
        include: /css/,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      title: "convoluted mirror",
      template: "./app//html/index.html",
    }),
  ],
  output: {
    filename: "[name].js",
    path: path.resolve(__dirname, "dist"),
    // Avoid MD4 which is disabled in Node >=17/OpenSSL 3
    hashFunction: "sha256",
  },
  experiments: {
    asyncWebAssembly: true,
  },
}
