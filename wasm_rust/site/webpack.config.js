const path = require("path");
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
  devServer: {
    static: {
      directory: path.resolve(__dirname, "dist"), // 指定静态文件目录
    },
    open: {
      target: ["index"], // 启动时打开 index.html 页面
    },
    port: 8080, // 可选，默认是 8080
  },
};
