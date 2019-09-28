import * as shell from 'shelljs'

shell.config.fatal = true
shell.config.verbose = true

shell.exec('tar -xzf micro-chimp.tar.gz --directory ~/.docker/machines/micro-chimp')
shell.sed('-i', '{{replace_username}}', shell.exec('whoami'), '~/.docker/machines/micro-chimp/config.json')
shell.exec('eval $(docker-machine env micro-chimp)')
