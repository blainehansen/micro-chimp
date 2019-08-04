import fs from 'fs'

const base_dockerfile = require('../docker/micro-chimp.Dockerfile').default
const postgres_dockerfile = require('../docker/postgres.Dockerfile').default
const nginx_dockerfile = require('../docker/nginx.Dockerfile').default
const docker_compose_yml = require('../docker/docker-compose.yml').default
const site_names_yml = require('../docker/site_names.yml').default
const postgres_schema_sql = require('../docker/postgres.schema.sql').default

const create_secrets_sh = require('../docker/create_secrets.sh').default
const create_machine_sh = require('../docker/create_machine.sh').default
const unpack_machine_sh = require('../docker/unpack_machine.sh').default
const destroy_machine_sh = require('../docker/destroy_machine.sh').default

fs.writeFileSync('micro-chimp.Dockerfile', base_dockerfile)
fs.writeFileSync('postgres.Dockerfile', postgres_dockerfile)
fs.writeFileSync('nginx.Dockerfile', nginx_dockerfile)
fs.writeFileSync('docker-compose.yml', docker_compose_yml)
fs.writeFileSync('site_names.yml', site_names_yml)
fs.writeFileSync('postgres.schema.sql', postgres_schema_sql)

fs.writeFileSync('create_secrets.sh', create_secrets_sh)
fs.writeFileSync('create_machine.sh', create_machine_sh)
fs.writeFileSync('unpack_machine.sh', unpack_machine_sh)
fs.writeFileSync('destroy_machine.sh', destroy_machine_sh)
