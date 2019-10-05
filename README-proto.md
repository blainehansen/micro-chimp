# `micro-chimp`

This project is an extremely simple and minimal email subscription server. It has all the code needed to deploy an email server to a docker-machine managed digital ocean droplet. A single deployed server can be used by multiple websites at the same time. The server has these endpoints:

- `/new-email`: Receive requests for new email subscriptions
- `/verify-email`: Verify emails with a verification token.
- `/unsubscribe`: Unsubscribe an email.

This server is written in asynchronous rust, which means it's incredibly high-performance, and rock solid reliable. The backing database is postgres, and mailgun is used for sending emails.

To get a server set up from start to finish, just follow these steps:

```sh
# make sure you have some dependencies
which docker
which docker-machine
which docker-compose
which git-secret # https://git-secret.io/installation

# set up a directory with configs, secrets, and management files
# this will include things like a docker-compose.yml,
# and a .env file that will be encrypted using `git-secret`
npx -p micro-chimp init ./my-deployment-directory

# `init` creates some files you'll have to fill in

# contains descriptions of the sites that will be managed by this server
# you'll need to fill this in with the real sites you want to manage
cat my-deployment-directory/sites_manifest.yml
# contains your secrets, such as database passwords and api keys
# most of these have already been filled with cryptographically strong tokens
# but two, MAILGUN_API_KEY and DIGITAL_OCEAN_KEY need to be filled in
# the file has comments describing what they're for and where to get them
ls my-deployment-directory/.env

# now that you've filled in all the right values,
# you're ready to deploy!

# creates a droplet that will run your server
npx -p micro-chimp create-machine

# deploy!
# the default is to do a staging deploy that won't actually provision any certificates
npx -p micro-chimp deploy --email person@example.com
# add the --live switch to do it for real
npx -p micro-chimp deploy --email person@example.com --live

# tears down the droplet
npx -p micro-chimp destroy-machine

# used to unzip the machine config files and install them locally
# this lets you manage the same server from different machines
# this is safe since the machine config archive file is encrypted with `git-secret`
npx -p micro-chimp unpack-machine
```
