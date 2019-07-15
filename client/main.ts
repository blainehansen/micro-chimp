import YAML from 'yaml'
import { Client } from 'pg'
import snake_case from 'snake-case'

function generate_enum() {
	//
}

// send out an email, creating an unsubscribe token a the same time
async function main() {
	const client = new Client()
	await client.connect()

	//

	await client.end()
}




async function send_message(client: Client) {
	const res = await client.query('select $1::text as message', ['hello world!'])
	console.log(res.rows[0].message)

	client.query(`insert into unsubscribe_token (site_name, token, description) values ($1::site_name_enum, $2, $3)`)
}
