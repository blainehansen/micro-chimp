import path from 'path'
import webpack from 'webpack'

export default {
	target: 'node',
	entry: {
		client: './bin/client.ts',
		codegen: './bin/codegen.ts',
		setup: './bin/setup.ts',
	},
	output: {
		path: path.join(__dirname, 'dist'),
		filename: '[name].js',
	},

	resolve: {
		extensions: ['.js', '.ts'],
	},

	module: {
		rules: [
			{ test: /\.ts$/, loader: 'ts-loader', exclude: [/node_modules/, /test/] },
			{ test: /(\.sql|\.yml|\.Dockerfile|\.sh|\.conf)$/, loader: 'raw-loader' },
		]
	},

	mode: 'none',
} as webpack.Configuration