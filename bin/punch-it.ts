import { JsonDecoder, Result, Err } from 'ts.data.json'
// import * as inquirer from 'inquirer'
import shell from 'shelljs'
import parseArgs from 'minimist'

const help_string = `
micro-chimp punch-it usage:
	--help				Display this message.
	--live				Request letsencrypt certificates in production mode (default is false, so you can test things and not hit the letsencrypt rate limits).
	--email=<your@email.com>	Provide an email that will be used to register for your letsencrypt certificates, and won't be shared with anyone. Required if you are running in live mode.
`

function fail() {
	console.error("Invalid arguments!")
	console.error(help_string)
	process.exit(1)
	return new Error("unreachable")
}

const args = parseArgs(process.argv.slice(2), {
	boolean: ['live', 'help'],
	string: 'email',
	default: { live: false, help: false, email: undefined },
})

const unrecognized = Object.keys(args).filter(key => !['_', 'live', 'help', 'email'].includes(key))
if (unrecognized.length > 0)
	throw fail()

const options_result = JsonDecoder.object(
	{
		live: JsonDecoder.boolean,
		help: JsonDecoder.boolean,
		email: JsonDecoder.oneOf<string | undefined>(
			[JsonDecoder.string, JsonDecoder.isUndefined(undefined)],
			'email',
		),
	},
	'options object',
).decode(args)

if (options_result instanceof Err)
	throw fail()

const options = options_result.value

if (options.help) {
	console.info(help_string)
	process.exit()
}

console.log(options)
