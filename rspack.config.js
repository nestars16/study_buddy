// @ts-check
//
const isProduction = process.env.ENV_MODE === "production";

/** @type {import('@rspack/cli').Configuration} */
module.exports = {
  context: __dirname,
  entry: {
    home: "./static_sources/js/index.js",
    recovery: "./static_sources/js/recovery.js",
  },
  mode: isProduction ? "production" : "development",
  output: {
    path: "./static/js/",
  },
};
