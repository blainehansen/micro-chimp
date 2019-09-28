import * as fs from 'fs'
import * as path from 'path'
import * as shell from 'shelljs'
import * as dotenv from 'dotenv'

shell.config.fatal = true
shell.config.verbose = true

const buf = fs.readFileSync('./.env')
const config = dotenv.parse(buf) as { [key: string]: string }

const digital_ocean_key = (config['DIGITAL_OCEAN_KEY'] || '').trim()
if (!digital_ocean_key) {
	shell.echo('DIGITAL_OCEAN_KEY is empty')
	shell.exit(1)
}

shell.exec(
	`docker-machine create --driver=digitalocean \
		--digitalocean-access-token=${digital_ocean_key} \
		--digitalocean-image=coreos-stable \
		--digitalocean-region=sfo2 \
		--digitalocean-size=1GB \
		--digitalocean-ssh-user=core \
		micro-chimp`,
)

shell.cp('-R', '~/.docker/machine/machines/micro-chimp', '.')
shell.mkdir('-p', './micro-chimp/certs')
shell.cp('-R', '~/.docker/machine/certs/*', './micro-chimp/certs')

shell.sed('-i', 'machine/certs', 'machine/machines/micro-chimp/certs', './micro-chimp/config.json')
shell.sed('-i', shell.exec('whoami'), '{{replace_username}}', './micro-chimp/config.json')

shell.exec('tar -zcf micro-chimp.tar.gz ./micro-chimp')
shell.exec('git secret add micro-chimp.tar.gz')
shell.exec('git secret hide')
shell.rm('-R', './micro-chimp')
