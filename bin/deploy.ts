import shell from 'shelljs'
import parse_args from 'minimist'
import { JsonDecoder, Result, Err } from 'ts.data.json'

import { parse_sites } from './codegen'
import { get_dir } from './utils'

const dir = get_dir()

shell.config.fatal = true
shell.config.verbose = true

const help_string = `\
micro-chimp deploy usage:
	--help				Display this message.
	--live				Request letsencrypt certificates in production mode (default is false, so you can test things and not hit the letsencrypt rate limits).
	--email=<your@email.com>	Provide an email that will be used to register for your letsencrypt certificates, and won't be shared with anyone. Required if you are running in live mode.
`

function fail() {
	shell.echo("Invalid arguments!")
	shell.echo(help_string)
	process.exit(1)
	return new Error("unreachable")
}

const args = parse_args(process.argv.slice(2), {
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

if (options_result.value.help) {
	shell.echo(help_string)
	process.exit(0)
}


const { live, email } = options_result.value

const sites = parse_sites()
const domain_args = Object.keys(sites).map(site_url => `-d subscriptions.${site_url}`)

const [env_args, _] = live
	? [
		`--email ${email} --no-eff-email`,
		shell.echo("Doing it live!"),
	]
	: [
		'--staging --register-unsafely-without-email',
		shell.echo("Just doing a test run."),
	]

shell.pushd(dir)

shell.exec('eval $(docker-machine env micro-chimp)')
shell.exec(`docker-compose build`)

shell.exec(
	`docker-compose run --rm --entrypoint " \
		openssl dhparam -out /etc/letsencrypt/dhparam-2048.pem 2048" certbot`
)


const cert_dir_name = "micro-chimp-domains"
const cert_path = `/etc/letsencrypt/live/${cert_dir_name}`
shell.echo(`creating fake cert at: ${cert_path}`)
shell.exec(`docker-compose run --rm --entrypoint "mkdir -p ${cert_path}" certbot`)
shell.exec(
	`docker-compose run --rm --entrypoint " \
		openssl req -x509 -nodes -newkey rsa:1024 -days 1 \
			-keyout '${cert_path}/privkey.pem' \
			-out '${cert_path}/fullchain.pem' \
			-subj '/CN=localhost'" certbot`
)

shell.echo("starting nginx")
shell.exec(`docker-compose up --force-recreate -d nginx`)

shell.echo("deleting fake certs")
shell.exec(
	`docker-compose run --rm --entrypoint " \
		rm -Rf /etc/letsencrypt/live/${cert_dir_name} && \
		rm -Rf /etc/letsencrypt/archive/${cert_dir_name} && \
		rm -Rf /etc/letsencrypt/renewal/${cert_dir_name}.conf" certbot`
)

shell.exec(
	`docker-compose run --rm --entrypoint " \
		certbot certonly
			--webroot -w /var/www/certbot \
			${domain_args} \
			--cert-name ${cert_dir_name} \
			${env_args} \
			--rsa-key-size 4096 \
			--agree-tos \
			--force-renewal" certbot`
)

shell.echo("reloading nginx")
shell.exec(`docker-compose exec nginx nginx -s reload`)

shell.exec(`docker-compose up -d`)
shell.exec(`docker-compose logs -f --timestamps`)


shell.popd()
