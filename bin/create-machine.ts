import * as fs from 'fs'
import * as path from 'path'
import * as shell from 'shelljs'
import * as dotenv from 'dotenv'
import { make_dir_path } from './utils'

shell.config.fatal = true
shell.config.verbose = true

const p = make_dir_path()

const buf = fs.readFileSync(p('./.env'))
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

shell.cp('-R', '~/.docker/machine/machines/micro-chimp', p('.'))
shell.mkdir('-p', p('./micro-chimp/certs'))
shell.cp('-R', '~/.docker/machine/certs/*', p('./micro-chimp/certs'))

shell.sed('-i', 'machine/certs', 'machine/machines/micro-chimp/certs', p('./micro-chimp/config.json'))
shell.sed('-i', shell.exec('whoami'), '{{replace_username}}', p('./micro-chimp/config.json'))

shell.exec(`tar -zcf ${p('micro-chimp.tar.gz')} ${p('./micro-chimp')}`)
shell.exec(`git secret add ${p('micro-chimp.tar.gz')}`)
shell.exec('git secret hide')
shell.rm('-R', p('./micro-chimp'))
