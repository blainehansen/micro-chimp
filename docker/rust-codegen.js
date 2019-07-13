const fs = require('fs')
const YAML = require('yaml')
const snake_case = require('snake-case')

const sites = YAML.parse(fs.readFileSync(process.argv[2], 'utf-8'))

if (sites.length === 0) throw new Error("")

const SITE_FIELDS_TEMPLATE = `{enum_name} => (
				"https://api.mailgun.net/v3/{site_url}/messages".to_string(),
				"{string_name}".to_string(),
				MailgunForm {
					to,
					from: "<{from_email}>".to_string(),
					subject: "{subject_text}".to_string(),
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

const file_string = `use serde::Deserialize;

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub enum SiteName {
	${site_names.join(',\n')}
}

use super::MailgunForm;

impl SiteName {
	pub fn get_site_information(&self, to: String, token: &str) -> (String, String, MailgunForm) {
		use SiteName::*;
		match *self {
			${site_fields.join('\n')}
		}
	}
}
`

fs.writeFileSync('sites.rs', file_string)