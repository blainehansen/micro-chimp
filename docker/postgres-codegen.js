const fs = require('fs')
const YAML = require('yaml')
const snake_case = require('snake-case')

const sites = YAML.parse(fs.readFileSync(process.argv[2], 'utf-8'))

if (sites.length === 0) throw new Error("")

const allowed_names = Object.keys(sites).map(k => `'${snake_case(k)}'`).join(', ')

const file_string = `create table emails (
	email citext unique check (email ~* '^.+@.+\..+$'),
	site_name text not null check (site_name in (${allowed_names})),
	validation_token text unique
);`

fs.writeFileSync('init.sql', file_string)
