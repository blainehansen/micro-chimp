import fs from 'fs'

const base_dockerfile = fs.readFileSync('./docker/micro-chimp.Dockerfile', 'utf-8')
const postgres_dockerfile = fs.readFileSync('./docker/postgres.Dockerfile', 'utf-8')
const nginx_dockerfile = fs.readFileSync('./docker/nginx.Dockerfile', 'utf-8')
const docker_compose_yml = fs.readFileSync('./docker/docker-compose.yml', 'utf-8')
const sites_manifest_yml = fs.readFileSync('./docker/sites_manifest.yml', 'utf-8')

const init_contents = fs.readFileSync('./bin/init-raw.ts', 'utf-8')

fs.writeFileSync('./bin/init.ts', [
	`const base_dockerfile = ${JSON.stringify(base_dockerfile)}`,
	`const postgres_dockerfile = ${JSON.stringify(postgres_dockerfile)}`,
	`const nginx_dockerfile = ${JSON.stringify(nginx_dockerfile)}`,
	`const docker_compose_yml = ${JSON.stringify(docker_compose_yml)}`,
	`const sites_manifest_yml = ${JSON.stringify(sites_manifest_yml)}`,
	init_contents,
].join('\n\n'))
