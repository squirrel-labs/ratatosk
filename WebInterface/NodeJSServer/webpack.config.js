const path = require('path');
const nodeExternals = require('webpack-node-externals');

module.exports = {
  mode: 'production',
  entry: {
    index: './src/index.js',
    about: './src/about.js',
    play: './src/play.js',
    playModule: './src/play-module.js',
  },
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, './dist/script'),
  },
};
