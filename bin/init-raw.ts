import fs from 'fs'
import shell from 'shelljs'
import crypto from 'crypto'

shell.config.fatal = true
shell.config.verbose = true

const dir = process.argv[2] || '.'

shell.mkdir('-p', dir)

shell.pushd(dir)

fs.writeFileSync('micro-chimp.Dockerfile', base_dockerfile)
fs.writeFileSync('postgres.Dockerfile', postgres_dockerfile)
fs.writeFileSync('nginx.Dockerfile', nginx_dockerfile)
fs.writeFileSync('docker-compose.yml', docker_compose_yml)
if (!fs.existsSync('sites_manifest.yml'))
	fs.writeFileSync('sites_manifest.yml', sites_manifest_yml)

function randomToken() {
	return crypto.randomBytes(128).toString('base64')
}

if (!fs.existsSync('.env')) {
	fs.writeFileSync('.env', [
		"# Go to the digital ocean api key page:",
		"# https://cloud.digitalocean.com/settings/api/tokens",
		"# generate a new key, and put it here.",
		"DIGITAL_OCEAN_KEY=",
		"",
		"# Go to the mailgun api security page:",
		"# https://app.mailgun.com/app/account/security/api_keys",
		"# Copy your Private Api Key, and put it here in this format:",
		"# api:[secret-key-goes-here]",
		"MAILGUN_API_KEY=",
		"",
		"POSTGRES_USER=admin_user",
		`POSTGRES_PASSWORD=${randomToken()}`,
		`SERVER_POSTGRES_PASSWORD=${randomToken()}`,
	].join('\n'))
}

shell.popd()

shell.echo("Initializing this repo with git-secret, and adding this current user to the allowed users.")
shell.config.fatal = false
shell.exec("git secret init")
shell.exec("git secret tell -m")
shell.config.fatal = true


shell.exec("git secret add sites_manifest.yml .env")
shell.exec("git secret hide")

shell.echo('sites_manifest.yml').toEnd('.gitignore')
shell.echo('.env').toEnd('.gitignore')
shell.echo('micro-chimp.tar.gz').toEnd('.gitignore')



const package_json = JSON.parse(fs.readFileSync('./package.json', 'utf-8'))
package_json['micro-chimp-dir'] = dir
package_json['husky'] = {
	...(package_json['husky'] || {}),
}
package_json['husky']['hooks'] = {
	...(package_json['husky']['hooks'] || {}),
	"pre-commit": "git secret hide && git add -u",
}
fs.writeFileSync('./package.json', JSON.stringify(package_json))

shell.echo("Installing husky npm package for pre-commit git hook, used to make sure `git-secret hide` is called")
shell.exec("npm install --save-dev husky")

