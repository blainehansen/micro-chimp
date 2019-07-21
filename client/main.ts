import YAML from 'yaml'
import { Client } from 'pg'
import snake_case from 'snake-case'

const sites = YAML.parse(fs.readFileSync(process.argv[2], 'utf-8'))

// send out an email, creating an unsubscribe token a the same time
async function main() {
	const client = new Client()
	await client.connect()

	//

	await client.end()
}


type SiteName = { [url: string]: { from_email: string } }

async function send_message<S extends SiteName>(
	client: Client, sites: S, site_name: keyof S, description: string,
	from_email: string, subject: string, body: string,
) {
	const random_token = generate_random_token()

	await client.query(
		`insert into unsubscribe_token (site_name, token, description) values ($1::site_name_enum, $2, $3)`,
		[site_name, random_token, description],
	)

	// await mailgun_client.bulk_send({

	// })
}
