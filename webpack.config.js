const path = require('path')

const config = {
	target: 'node',
	entry: {
		'micro-chimp': './bin/micro-chimp.ts',
		'micro-chimp-cli': './client/main.ts',
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
			{ test: /(\.yml|\.Dockerfile)$/, loader: 'raw-loader' },
		]
	},

	mode: 'none',
}

module.exports = config
