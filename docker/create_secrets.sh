# curl -L https://raw.githubusercontent.com/wmnnd/nginx-certbot/master/init-letsencrypt.sh > init-letsencrypt.sh

git secret init

git secret tell -m

touch \
	.secret.mailgun_auth_file \
	.secret.digital_ocean_key \
	.secret.admin_postgres_user_file \
	.secret.admin_postgres_password_file \
	.secret.server_postgres_password_file

echo 'Replace these files contents with your mailgun key, in this format: "api:secret-key".' > .secret.mailgun_auth_file

cat << EOF > .secret.digital_ocean_key
Go to the digital ocean api key page:
https://cloud.digitalocean.com/settings/api/tokens

Generate a new key, and replace these file contents with that key.
EOF

echo 'admin_user' > .secret.admin_postgres_user_file

openssl rand -base64 64 | tr -d "[:space:]" > .secret.admin_postgres_password_file
openssl rand -base64 64 | tr -d "[:space:]" > .secret.server_postgres_password_file

cat << EOF >> .gitignore

.secret.*

EOF

git secret add  \
	.secret.mailgun_auth_file \
	.secret.digital_ocean_key \
	.secret.admin_postgres_user_file \
	.secret.admin_postgres_password_file \
	.secret.server_postgres_password_file

git secret hide
