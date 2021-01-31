const path = require("path")
const HtmlWebpackPlugin = require("html-webpack-plugin")
const { CleanWebpackPlugin } = require("clean-webpack-plugin")

module.exports = {
  mode: "none",
  entry: {
    index: "./index.js",
  },
  devServer: {
    contentBase: path.join(__dirname, "dist"),
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
  },
  experiments: {
    asyncWebAssembly: true,
  },
}
