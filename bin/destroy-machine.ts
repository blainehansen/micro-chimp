import shell from 'shelljs'

shell.config.fatal = true
shell.config.verbose = true

shell.exec('docker-machine rm micro-chimp')
