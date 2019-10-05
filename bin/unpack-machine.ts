import * as shell from 'shelljs'
import { make_dir_path } from './utils'

shell.config.fatal = true
shell.config.verbose = true

const p = make_dir_path()

shell.exec(`tar -xzf ${p('micro-chimp.tar.gz')} --directory ~/.docker/machines/micro-chimp`)
shell.sed('-i', '{{replace_username}}', shell.exec('whoami'), '~/.docker/machines/micro-chimp/config.json')
shell.exec('eval $(docker-machine env micro-chimp)')
