import fs from 'fs'
import YAML from 'yaml'
import snake_case from 'snake-case'
import { JsonDecoder, Err } from 'ts.data.json'

type SiteDeclarations = {
	[site_url: string]: {
		from_email: string,
		subject_text: string,
		body_text: string,
	}
}

const site_declaration_decoder = JsonDecoder.dictionary(
	JsonDecoder.object({
		from_email: JsonDecoder.string,
		subject_text: JsonDecoder.string,
		body_text: JsonDecoder.string,
	}, 'SiteDeclaration'),
	'SiteDeclarations',
)


const site_names_file = process.argv[2]
if (!site_names_file) throw new Error("setup requires a site names yml file")
const email = process.argv[3]
if (!email) throw new Error("setup requires a registration email for certbot (which won't be shared with them)")

const sites_result = site_declaration_decoder.decode(YAML.parse(fs.readFileSync(site_names_file, 'utf-8')))
if (sites_result instanceof Err) throw new Error(`your site_names yaml file wasn't formatted correctly: ${sites_result.error}`)
const sites = sites_result.value

if (Object.keys(sites).length === 0) throw new Error("You haven't included any sites in your yml file!")



const allowed_names = Object.keys(sites).map(site_url => `'${snake_case(site_url)}'`).join(', ')
const schema_file_string = `create type site_name_enum as enum(${allowed_names});`
fs.writeFileSync('site_name_enum.sql', schema_file_string)



const SITE_FIELDS_TEMPLATE = `{enum_name} => (
				"https://api.mailgun.net/v3/{site_url}/messages",
				"{string_name}",
				MailgunForm {
					to,
					from: "<{from_email}>",
					subject: "{subject_text}",
					text: {body_text}.replace("{verification_token}", token),
				},
			),`

const site_names = []
const site_fields = []
for (const [site_url, { from_email, subject_text, body_text }] of Object.entries(sites)) {
	const site_name = snake_case(site_url)

	site_names.push(site_name)

	site_fields.push(
		SITE_FIELDS_TEMPLATE
			.replace('{enum_name}', site_name)
			.replace('{site_url}', site_url)
			.replace('{string_name}', site_name)
			.replace('{from_email}', from_email)
			.replace('{subject_text}', subject_text)
			.replace('{body_text}', JSON.stringify(body_text))
	)
}

const rust_file_string = `use serde::Deserialize;

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub enum SiteName {
	${site_names.map(name => `${name},`).join('\n')}
}

use super::MailgunForm;

impl SiteName {
	pub fn get_site_information(self, to: String, token: &str) -> (&'static str, &'static str, MailgunForm) {
		use SiteName::*;
		match self {
			${site_fields.join('\n\t\t\t')}
		}
	}
}
`

fs.writeFileSync('sites.rs', rust_file_string)



const domain_names = Object.keys(sites).map(site_url => `subscriptions.${site_url}`)

const secure_nginx_certificates = domain_names.map(site_url => `server {
	include /etc/nginx/includes/secure;
	server_name ${site_url};
	ssl_certificate /etc/letsencrypt/live/${site_url}/fullchain.pem;
	ssl_certificate_key /etc/letsencrypt/live/${site_url}/privkey.pem;
}`).join('\n\n')

const nginx_conf = `server {
	include /etc/nginx/includes/normal;
	server_name ${domain_names};
}

${secure_nginx_certificates}
`

fs.writeFileSync('nginx.conf', nginx_conf)



const DEPLOY_TEMPLATE = `eval $(docker-machine env micro-chimp)
eval $(cat .secret.postgres.env)
export MAILGUN_AUTH=$(tr -d "[:space:]" < .secret.mailgun_auth)

docker-compose build

docker-compose run --rm --entrypoint " \\
	openssl dhparam -out /etc/letsencrypt/dhparam-2048.pem 2048" certbot

domains=(${domain_names})

for domain in "\${domains[@]}"; do
	path="/etc/letsencrypt/live/$domain"
	docker-compose run --rm --entrypoint "mkdir -p $path" certbot

	docker-compose run --rm --entrypoint " \\
		openssl req -x509 -nodes -newkey rsa:1024 -days 1\\
			-keyout '$path/privkey.pem' \\
			-out '$path/fullchain.pem' \\
			-subj '/CN=localhost'" certbot
done

docker-compose up --no-deps --force-recreate -d nginx

domain_args=""
for domain in "\${domains[@]}"; do

	docker-compose run --rm --entrypoint " \\
		rm -Rf /etc/letsencrypt/live/$domain && \\
		rm -Rf /etc/letsencrypt/archive/$domain && \\
		rm -Rf /etc/letsencrypt/renewal/$domain.conf" certbot

  domain_args="$domain_args -d $domain"
done

docker-compose run --rm --entrypoint " \\
	certbot certonly --webroot -w /var/www/certbot \\
		{staging_argument} \\
		{email_argument} \\
		$domain_args \\
		--rsa-key-size 4096 \\
		--agree-tos \\
		--force-renewal" certbot

docker-compose up --force-recreate -d
`

fs.writeFileSync(
	'deploy.testing.sh',
	DEPLOY_TEMPLATE
		.replace('{staging_argument}', '--staging')
		.replace('{email_argument}', '--register-unsafely-without-email'),
)
fs.writeFileSync(
	'deploy.production.sh',
	DEPLOY_TEMPLATE
		.replace('{staging_argument}', '')
		.replace('{email_argument}', `--email ${email} --no-eff-email`),
)
