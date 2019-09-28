import * as shell from 'shelljs'

shell.config.fatal = true
shell.config.verbose = true

// const dir = process.argv[2] || '.'

// function p(path: string) {
// 	return path.join(dir, path)
// }

shell.exec('tar -xzf .secret.micro-chimp.tar.gz --directory ~/.docker/machines/micro-chimp')

shell.sed('-i', '{{replace_username}}', shell.exec('whoami'), '~/.docker/machines/micro-chimp/config.json')

shell.exec('eval $(docker-machine env micro-chimp)')
