const fs = require('fs')
const YAML = require('yaml')
// const data = fs.readFileSync("/dev/stdin", "utf-8").trim()

const sites = YAML.parse(fs.readFileSync(process.argv[2], 'utf-8'))

if (sites.length === 0) throw new Error("")

const use_site_name = sites.length > 1

const insert_args = use_site_name
	? "(email, validation_token, site_name) values ($1, $2, $3)"
	: "(email, validation_token) values ($1, $2)"

const site_name_field = use_site_name
	? "site_name: String,"
	: ""


const file_string = `
pub const NEW_EMAIL_QUERY = "insert into emails ${insert_args}";
pub const VERIFY_QUERY = "update emails set validation_token = null where validation_token = $1";

pub const MAILGUN_URL = "http://api.mailgun.net/v3/${host_url}/messages";

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEmailMessage {
	email: String,
	${site_name_field}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyEmailInput {
	encoded_validation_token: String,
}
`

fs.writeFileSync('thing.rs', file_string)
