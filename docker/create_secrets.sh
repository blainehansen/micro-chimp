# curl -L https://raw.githubusercontent.com/wmnnd/nginx-certbot/master/init-letsencrypt.sh > init-letsencrypt.sh

git secret init

git secret tell -m

touch \
	.secret.mailgun_auth \
	.secret.digital_ocean_key \
	.secret.postgres.env

if [ ! -f .secret.mailgun_auth ]; then
cat << EOF > .secret.mailgun_auth
Go to the mailgun api security page:
https://app.mailgun.com/app/account/security/api_keys

Copy your Private Api Key, and replace these file contents with that key, in this format:
api:the-secret-key blah
EOF
fi

if [ ! -f .secret.digital_ocean_key ]; then
cat << EOF > .secret.digital_ocean_key
Go to the digital ocean api key page:
https://cloud.digitalocean.com/settings/api/tokens

Generate a new key, and replace these file contents with that key.
EOF
fi

cat << EOF > .secret.postgres.env
export POSTGRES_USER=admin_user
export POSTGRES_PASSWORD=$(openssl rand -base64 64 | tr -d "[:space:]")
export SERVER_POSTGRES_PASSWORD=$(openssl rand -base64 64 | tr -d "[:space:]")
EOF


cat << EOF >> .gitignore
.secret.*
!.secret.*.secret
site_names.yml
sites.rs
site_name_enum.sql
nginx.conf
deploy.*.sh
EOF

git secret add  \
	site_names.yml \
	sites.rs \
	site_name_enum.sql \
	nginx.conf \
	deploy.*.sh \
	.secret.mailgun_auth \
	.secret.digital_ocean_key \
	.secret.postgres.env

git secret hide
