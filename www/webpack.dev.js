const CopyWebpackPlugin = require('copy-webpack-plugin');
const path = require('path');

module.exports = {
    mode: 'development',
    entry: './bootstrap.ts',
    devtool: 'inline-source-map',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bootstrap.js',
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.ts', '.js', '.wasm'],
    },
    plugins: [
        new CopyWebpackPlugin([
            'index.html',
            { from: 'node_modules/bulma/css/bulma.min.css', to: 'node_modules/bulma/css/' },
            'style.css',
            { from: 'assets', to: 'assets' },
        ]),
    ],
};
