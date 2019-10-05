import fs from 'fs'
import shell from 'shelljs'

export function get_dir() {
	const package_json = JSON.parse(fs.readFileSync('./package.json', 'utf-8'))
	const dir = package_json['micro-chimp-dir']
	if (!dir) {
		shell.echo('need a deployments directory set in package.json["micro-chimp-dir"]')
		shell.exit(1)
	}

	if (!fs.existsSync(dir)) {
		shell.echo(`the deployments directory set in package.json["micro-chimp-dir"] doesn't exist`)
		shell.exit(1)
	}

	return dir
}
