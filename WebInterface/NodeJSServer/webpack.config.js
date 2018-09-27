const path = require('path');
const nodeExternals = require('webpack-node-externals');

module.exports = {
  target: 'node',
  mode: 'production',
  entry: {
    server: './src/server.js',
    index: './src/index.js'
  },
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, './dist')
  },
  node: {
    '__dirname': false,
  },
  externals: [nodeExternals({
    modulesFromFile: true
  })],
};
