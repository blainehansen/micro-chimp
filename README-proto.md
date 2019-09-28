# `micro-chimp`

This project is an extremely simple and minimal email subscription server. It has all the code needed to deploy an email server to a docker-machine managed digital ocean droplet. A single deployed server can be used by multiple websites at the same time. The server has these endpoints:

- `/new-email`: Receive requests for new email subscriptions
- `/verify-email`: Verify emails with a verification token.
- `/unsubscribe`: Unsubscribe an email.

This server is written in asynchronous rust, which means it's incredibly high-performance, and rock solid reliable. The backing database is postgres, and mailgun is used for sending emails.

To get a server set up from start to finish, just follow these steps:

```sh
# set up a directory with the configs, secrets, and management files
# this will include things like a docker-compose.yml,
# and several secrets files
npx -p micro-chimp init [deployment-management-directory-name = .]

# several files should have been created for you to inspect and fill with values

# contains descriptions of the sites that will be managed by this server
# you'll need to fill this in with the real sites you want to manage
cat site_names.yml
# needs to be filled in with a key
# used to create the droplet where your server will run
cat .secret.digital_ocean_key
# needs to be filled in with your mailgun api key
# used to send verification emails from the server
cat .secret.mailgun_auth
# contains user names and passwords for your postgres database
# should have been filled with strong random passwords using `openssl`
ls .secret.postgres.env
```

```sh
# creates a droplet
./create_machine.sh

```
