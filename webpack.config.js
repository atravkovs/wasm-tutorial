const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
    mode: 'development',
    entry: './src/index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        globalObject: 'this'
    },
    devServer: {
        open: true,
        host: '0.0.0.0',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html',
        }),
        new WasmPackPlugin({
            crateDirectory: path.join(__dirname, './rust'),
            outDir: path.join(__dirname, './pkg'),
        })
    ],
    module: {
        rules: [
            {
                test: /\.(eot|svg|ttf|woff|woff2|png|jpg|gif)$/,
                type: 'asset',
            },
        ],
    },
    experiments: {
        asyncWebAssembly: true,
    }
};
