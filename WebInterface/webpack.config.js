const path = require('path');
const nodeExternals = require('webpack-node-externals');

module.exports = [
  {
    mode: 'production',
    entry: {
      index: './src/js/index.js',
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
      about: './src/js/about.js',
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
      play: './src/js/play.js',
    },
    output: {
      filename: '[name].js',
      chunkFilename: '[name].js',
      path: path.resolve(__dirname, './dist/script/'),
      publicPath: '../script/',
    },
  },
];
