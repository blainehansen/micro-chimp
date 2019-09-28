live_flag=$1
email=$2

if ! [[ "$live_flag" =~ ^[0-9]+$ ]] || [ -z $live_flag ]; then
	echo "You need to give a live_flag of 0 or 1 as the first argument."
	exit 1
fi

if [ -z $email ]; then
	echo "You need to provide an email as the second argument. It will only be used to register you as the admin of your letsencrypt certificate."
	exit 1
fi

domains=("$@")
domains=("${domains[@]:2}")
domains_length=${#domains[@]}
if [ $domains_length -eq 0 ]; then
	echo "You need to provide some domains as the remaining arguments after the live_flag and your email."
	exit 1
fi

echo "Got arguments:"
echo "  live_flag: $live_flag"
echo "  email: $email"
echo "  domains: ${domains[@]}"
echo

domain_args=""
echo "Running for these domains:"
for domain in "${domains[@]}"; do
	echo "  $domain"
  domain_args="$domain_args -d $domain"
done
echo

env_args=""
if [ $live_flag -eq '1' ]; then
	echo "Doing it live!!! Using email: $email"
	env_args="--email $email --no-eff-email"
else
	echo "Just doing a test run."
	env_args="--staging --register-unsafely-without-email"
fi


eval $(docker-machine env micro-chimp)
eval $(cat .secret.postgres.env)
export MAILGUN_AUTH=$(tr -d "[:space:]" < .secret.mailgun_auth)

docker-compose build

docker-compose run --rm --entrypoint " \
	openssl dhparam -out /etc/letsencrypt/dhparam-2048.pem 2048" certbot


cert_dir_name="micro-chimp-domains"
cert_path="/etc/letsencrypt/live/$cert_dir_name"
echo "creating fake cert at: $cert_path"
docker-compose run --rm --entrypoint "mkdir -p $cert_path" certbot
docker-compose run --rm --entrypoint " \
	openssl req -x509 -nodes -newkey rsa:1024 -days 1\
		-keyout '$cert_path/privkey.pem' \
		-out '$cert_path/fullchain.pem' \
		-subj '/CN=localhost'" certbot


echo "starting nginx"
docker-compose up --force-recreate -d nginx


echo "deleting fake certs"
docker-compose run --rm --entrypoint " \
	rm -Rf /etc/letsencrypt/live/$cert_dir_name && \
	rm -Rf /etc/letsencrypt/archive/$cert_dir_name && \
	rm -Rf /etc/letsencrypt/renewal/$cert_dir_name.conf" certbot


docker-compose run --rm --entrypoint " \
	certbot certonly
		--webroot -w /var/www/certbot \
		$domain_args \
		--cert-name $cert_dir_name \
		$env_args \
		--rsa-key-size 4096 \
		--agree-tos \
		--force-renewal" certbot

echo "reloading nginx"
docker-compose exec nginx nginx -s reload

docker-compose up -d
docker-compose logs -f --timestamps
