import shell from 'shelljs'
import { get_dir } from './utils'

shell.config.fatal = true
shell.config.verbose = true

shell.pushd(get_dir())

shell.exec(`tar -xzf micro-chimp.tar.gz --directory ~/.docker/machines/micro-chimp`)
shell.sed('-i', '{{replace_username}}', shell.exec('whoami'), '~/.docker/machines/micro-chimp/config.json')
shell.exec('eval $(docker-machine env micro-chimp)')

shell.popd()
