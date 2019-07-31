import fs from 'fs'
import path from 'path'

const target_directory = process.argv[2] || './micro-chimp/'

fs.mkdirSync(target_directory, { recursive: true })

const base_docker_file = require('../docker/micro-chimp.Dockerfile').default
const postgres_docker_file = require('../docker/postgres.Dockerfile').default
const docker_compose_file = require('../docker/docker-compose.yml').default
const site_names_yml = require('../docker/site_names.yml').default

const create_secrets = require('../docker/create_secrets.sh').default
const create_machine = require('../docker/create_machine.sh').default
const deploy = require('../docker/deploy.sh').default

const clone_machine_config = require('../docker/clone_machine_config.sh').default
const unpack_and_connect_machine_config = require('../docker/unpack_and_connect_machine_config.sh').default
const destroy_machine = require('../docker/destroy_machine.sh').default

fs.writeFileSync(path.join(target_directory, 'micro-chimp.Dockerfile'), base_docker_file)
fs.writeFileSync(path.join(target_directory, 'postgres.Dockerfile'), postgres_docker_file)
fs.writeFileSync(path.join(target_directory, 'docker-compose.yml'), docker_compose_file)
fs.writeFileSync(path.join(target_directory, 'site_names.yml'), site_names_yml)

fs.writeFileSync(path.join(target_directory, 'create_secrets.sh'), create_secrets)
fs.writeFileSync(path.join(target_directory, 'create_machine.sh'), create_machine)
fs.writeFileSync(path.join(target_directory, 'deploy.sh'), deploy)

fs.writeFileSync(path.join(target_directory, 'clone_machine_config.sh'), clone_machine_config)
fs.writeFileSync(path.join(target_directory, 'unpack_and_connect_machine_config.sh'), unpack_and_connect_machine_config)
fs.writeFileSync(path.join(target_directory, 'destroy_machine.sh'), destroy_machine)
