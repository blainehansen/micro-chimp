const base_docker_file = require('../docker.micro-chimp.Dockerfile').default
const postgres_docker_file = require('../docker.postgres.Dockerfile').default
const docker_compose_file = require('../docker-compose.yml').default
const site_names_yml = require('../docker.site_names.yml').default

import fs from 'fs'
import path from 'path'

const target_directory = process.argv[2] || './micro-chimp/'

fs.mkdirSync(target_directory, { recursive: true })

fs.writeFileSync(path.join(target_directory, 'micro-chimp.Dockerfile'), base_docker_file)
fs.writeFileSync(path.join(target_directory, 'postgres.Dockerfile'), postgres_docker_file)
fs.writeFileSync(path.join(target_directory, 'docker-compose.yml'), docker_compose_file)
fs.writeFileSync(path.join(target_directory, 'site_names.yml'), site_names_yml)
