const fs = require('fs')
const YAML = require('yaml')
const snake_case = require('snake-case')

const sites = YAML.parse(fs.readFileSync(process.argv[2], 'utf-8'))

if (sites.length === 0) throw new Error("")

const allowed_names = Object.keys(sites).map(k => `'${snake_case(k)}'`).join(', ')

const file_string = `create type site_name_enum as enum(${allowed_names});`

fs.writeFileSync('schema_site_name_enum.sql', file_string)
