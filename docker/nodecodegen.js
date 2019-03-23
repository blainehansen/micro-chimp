const fs = require('fs')
const YAML = require('yaml')
const pascal_case = require('pascal-case')
const constant_case = require('constant-case')
// const data = fs.readFileSync("/dev/stdin", "utf-8").trim()

const sites = YAML.parse(fs.readFileSync(process.argv[2], 'utf-8'))

if (sites.length === 0) throw new Error("")

// const use_site_url = sites.length > 1

// const insert_args = use_site_url
// 	? "(email, validation_token, site_url) values ($1, $2, $3)"
// 	: "(email, validation_token) values ($1, $2)"

// const site_url_field = use_site_url
// 	? "site_url: String,"
// 	: ""


const SITE_FIELDS_TEMPLATE = `{pascal_name} => (
				"https://api.mailgun.net/v3/{site_url}/messages".to_string(),
				"{constant_name}".to_string(),
				MailgunForm {
					to,
					from: "<{from_email}>".to_string(),
					subject: "{subject_text}".to_string(),
					text: {body_text}.to_string(),
				},
			),`

const pascal_site_names = []
const site_fields = []
for (const [site_url, { from_email, subject_text, body_text }] of Object.entries(sites)) {
	const pascal_name = pascal_case(site_url)
	const constant_name = constant_case(site_url)

	pascal_site_names.push(pascal_name)

	site_fields.push(
		SITE_FIELDS_TEMPLATE
			.replace('{pascal_name}', pascal_name)
			.replace('{site_url}', site_url)
			.replace('{constant_name}', constant_name)
			.replace('{from_email}', from_email)
			.replace('{subject_text}', subject_text)
			.replace('{body_text}', JSON.stringify(body_text))
	)
}

const file_string = `use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum SiteName {
	${pascal_site_names.join(',\n')}
}

use super::MailgunForm;

impl SiteName {
	pub fn get_site_information(self, to: String) -> (String, String, MailgunForm) {
		use SiteName::*;
		match self {
			${site_fields.join('\n')}
		}
	}
}
`

fs.writeFileSync('sites.rs', file_string)
