// import qs from 'qs'
// import YAML from 'yaml'
// import axios from 'axios'
// import { Client } from 'pg'
// import Cursor from 'pg-cursor'
// import { promisify } from 'util'
// import snake_case from 'snake-case'

// Cursor.prototype.read_async = promisify(Cursor.prototype.read)

// const cursor = client.query(new Cursor('select * from generate_series(0, 5)'))
// let rows = await cursor.read_async(1000)
// while (rows.length) {
//   // do something with rows
//   rows = await cursor.read_async(1000)
// }
// // cursor.close(callback: (err: Error) => void) => void


// async function send_bulk() {
// 	const encoded_mailgun_auth = btoa(`api:${mailgun_auth}`)

// 	await axios.post(
// 		`https://api.mailgun.net/v3/${site_name}/messages`,
// 		{
// 		  headers: {
// 		  	'content-type': 'application/x-www-form-urlencoded',
// 		  	'authorization': `Basic ${encoded_mailgun_auth}`,
// 		  },
// 		  data: create_bulk_send_form(variables, from, subject, text),
// 		},
// 	)
// }


// type RecipientVariables = { [var_name: string]: string | number }
// type BulkSend<T extends RecipientVariables> = { [email: string]: T }

// function check_text<T extends RecipientVariables>(variables: T, subject: string, body: string): string[] {
// 	const errors = [] as string[]

// 	for (const key in variables) {
// 		// check that the key is somewhere in one of the texts
// 		const re = new RegExp(`%recipient.${key}%`)
// 		if (!re.test(subject) && !re.test(body)) errors.push(`key ${key} wasn't found in the subject or the body`)
// 	}

// 	return errors
// }

// function create_bulk_send_form<T extends RecipientVariables>(variables: T, from: string, subject: string, text: string): string {
// 	const errors = check_text()
// 	if (errors.length > 0) throw new Error(errors)

// 	return qs.stringify({
// 		from, subject, text,
// 		to: Object.keys(variables),
// 		'recipient-variables': variables,
// 	})
// }

// const sites = YAML.parse(fs.readFileSync(process.argv[2], 'utf-8'))

// // send out an email, creating an unsubscribe token a the same time
// async function main() {
// 	const client = new Client()
// 	await client.connect()

// 	//

// 	await client.end()
// }


// type SiteName = { [url: string]: { from_email: string } }

// async function send_message<S extends SiteName>(
// 	client: Client, sites: S, site_name: keyof S, description: string,
// 	from_email: string, subject: string, body: string,
// ) {
// 	const random_token = generate_random_token()

// 	await client.query(
// 		`insert into unsubscribe_token (site_name, token, description) values ($1::site_name_enum, $2, $3)`,
// 		[site_name, random_token, description],
// 	)

// 	// await mailgun_client.bulk_send({

// 	// })
// }
