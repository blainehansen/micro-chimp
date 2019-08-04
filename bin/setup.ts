import fs from 'fs'
import path from 'path'

const base_docker_file = require('../docker/micro-chimp.Dockerfile').default
const postgres_docker_file = require('../docker/postgres.Dockerfile').default
const nginx_docker_file = require('../docker/nginx.Dockerfile').default
const docker_compose_file = require('../docker/docker-compose.yml').default
const site_names_yml = require('../docker/site_names.yml').default

const create_secrets = require('../docker/create_secrets.sh').default
const create_machine = require('../docker/create_machine.sh').default
const deploy = require('../docker/deploy.sh').default

const clone_machine_config = require('../docker/clone_machine_config.sh').default
const unpack_and_connect_machine_config = require('../docker/unpack_and_connect_machine_config.sh').default
const destroy_machine = require('../docker/destroy_machine.sh').default

fs.writeFileSync('micro-chimp.Dockerfile', base_docker_file)
fs.writeFileSync('postgres.Dockerfile', postgres_docker_file)
fs.writeFileSync('nginx.Dockerfile', nginx_docker_file)
fs.writeFileSync('docker-compose.yml', docker_compose_file)
fs.writeFileSync('site_names.yml', site_names_yml)

fs.writeFileSync('create_secrets.sh', create_secrets)
fs.writeFileSync('create_machine.sh', create_machine)
fs.writeFileSync('deploy.sh', deploy)

fs.writeFileSync('clone_machine_config.sh', clone_machine_config)
fs.writeFileSync('unpack_and_connect_machine_config.sh', unpack_and_connect_machine_config)
fs.writeFileSync('destroy_machine.sh', destroy_machine)
