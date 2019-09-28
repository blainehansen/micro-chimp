import path from 'path'
import webpack from 'webpack'

export default {
	target: 'node',
	entry: {
		'bin/create-machine': './bin/create-machine.ts',
		'bin/destroy-machine': './bin/destroy-machine.ts',
		'bin/init': './bin/init.ts',
		'bin/send-mail': './bin/send-mail.ts',
		'bin/unpack-machine': './bin/unpack-machine.ts',
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
			{
				test: /(\.sql|\.yml|\.Dockerfile|\.sh|\.conf)$/,
				loader: 'raw-loader',
			},
		],
	},

	mode: 'none',
} as webpack.Configuration
