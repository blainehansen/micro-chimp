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
if (!email) throw new Error("setup requires a registration email for certbot (which won't be shared for them)")

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
			${site_fields.join('\n')}
		}
	}
}
`

fs.writeFileSync('sites.rs', rust_file_string)



const nginx_server_names = Object.keys(sites).map(site_url => `subscriptions.${site_url}`)

const nginx_certificates = nginx_server_names.map(site_url => `
	ssl_certificate /etc/letsencrypt/live/${site_url}/fullchain.pem;
	ssl_certificate_key /etc/letsencrypt/live/${site_url}/privkey.pem;
`.trim())

const nginx_conf = `upstream backend {
	server api:5050;
}

server {
	listen 80;
	listen [::]:80;
	server_name ${nginx_server_names};
	server_tokens off;

	location /.well-known/acme-challenge/ {
		root /var/www/certbot;
	}
}

server {
	listen 443 ssl;
	server_name ${nginx_server_names};
	server_tokens off;

	${nginx_certificates}

	include /etc/letsencrypt/options-ssl-nginx.conf;
	ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

	location / {
		proxy_pass http://backend;
		proxy_set_header    Host                $http_host;
		proxy_set_header    X-Real-IP           $remote_addr;
		proxy_set_header    X-Forwarded-For     $proxy_add_x_forwarded_for;

		# add_header X-Frame-Options "SAMEORIGIN" always;
		# add_header X-XSS-Protection "1; mode=block" always;
		# add_header X-Content-Type-Options "nosniff" always;
		# add_header Referrer-Policy "no-referrer-when-downgrade" always;
		# add_header Content-Security-Policy "default-src * data: 'unsafe-eval' 'unsafe-inline'" always;
	}
}
`

fs.writeFileSync('nginx.conf', nginx_conf)


const domain_names = nginx_server_names.map(server => `-d ${server}`).join(' ')

const DEPLOY_TEMPLATE = `eval $(docker-machine env micro-chimp)
docker-compose build
docker-compose up -d

docker-compose run --rm --entrypoint "\\
	certbot certonly --webroot -w /var/www/certbot \\
		{staging_argument} \\
		{email_argument} \\
		${domain_names} \\
		--rsa-key-size 4096 \\
		--agree-tos \\
		--force-renewal" certbot
`

fs.writeFileSync(
	'deploy.staging.sh',
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
