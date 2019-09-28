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
const comma_domain_names = domain_names.join(', ')

const secure_nginx_certificates = domain_names.map(site_url => ``).join('\n\n')

const nginx_conf = `server {
	include /etc/nginx/includes/normal;
	server_name ${comma_domain_names};
}

server {
	include /etc/nginx/includes/secure;
	server_name ${comma_domain_names};
}
`

fs.writeFileSync('nginx.conf', nginx_conf)



const DEPLOY_TEMPLATE = `source ./deploy.sh {live_flag} ${email} ${domain_names.join(' ')}`

fs.writeFileSync(
	'deploy.testing.sh',
	DEPLOY_TEMPLATE.replace('{live_flag}', '0')
)
fs.writeFileSync(
	'deploy.production.sh',
	DEPLOY_TEMPLATE.replace('{live_flag}', '1')
)
