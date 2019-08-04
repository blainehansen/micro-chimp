const path = require('path')

const config = {
	target: 'node',
	entry: {
		'setup': './bin/setup.ts',
		'client': './bin/client.ts',
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
			{ test: /(\.yml|\.Dockerfile|\.sh)$/, loader: 'raw-loader' },
		]
	},

	mode: 'none',
}

module.exports = config
