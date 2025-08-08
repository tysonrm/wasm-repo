const path = require('path')

const server = {
  target: 'async-node',
  mode: 'development',
  entry: path.resolve(__dirname, 'src/model.mjs'),
  output: {
    publicPath: 'http://localhost:8001/',
    path: path.resolve(__dirname, 'dist'),
    libraryTarget: 'commonjs',
    filename: 'model.js'
  }
}

module.exports = [server]
