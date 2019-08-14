import path from 'path'
import webpack from 'webpack'

export default {
	target: 'node',
	entry: {
		'bin/send-mail': './bin/send-mail.ts',
		'bin/codegen': './bin/codegen.ts',
		'bin/initialize': './bin/initialize.ts',
	},
	output: {
		path: path.join(__dirname, 'dist'),
		filename: '[name].js',
		libraryTarget: 'commonjs',
	},

	resolve: {
		extensions: ['.js', '.ts'],
	},

	module: {
		rules: [
			{
				test: /\.ts$/,
				loader: 'ts-loader',
				exclude: [/node_modules/, /test/],
			},
			{ test: /(\.sql|\.yml|\.Dockerfile|\.sh|\.conf)$/, loader: 'raw-loader' },
		]
	},

	mode: 'none',
} as webpack.Configuration
