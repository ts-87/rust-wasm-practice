const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: {
    gol: "./gol.js",
    rev: "./rev.js"
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].js",
  },
  mode: "development",
  //plugins: [
  //  new CopyWebpackPlugin(['index.html'])
  //],
};
