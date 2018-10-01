const path = require('path');
const nodeExternals = require('webpack-node-externals');

module.exports = [
  {
    mode: 'production',
    entry: {
      index: './src/index.js',
    },
    output: {
      filename: '[name].js',
      chunkFilename: '[name].js',
      path: path.resolve(__dirname, './dist/script/'),
      publicPath: './script/',
    },
  }, {
    mode: 'production',
    entry: {
      about: './src/about.js',
    },
    output: {
      filename: '[name].js',
      chunkFilename: '[name].js',
      path: path.resolve(__dirname, './dist/script/'),
      publicPath: '../script/',
    },
  }, {
    mode: 'production',
    entry: {
      play: './src/play.js',
    },
    output: {
      filename: '[name].js',
      chunkFilename: '[name].js',
      path: path.resolve(__dirname, './dist/script/'),
      publicPath: '../script/',
    },
  },
];
