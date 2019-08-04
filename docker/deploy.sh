eval $(docker-machine env micro-chimp)
docker-compose build
docker-compose up -d

docker-compose run --rm --entrypoint "\
  certbot certonly --webroot -w /var/www/certbot \
    --staging \
    --register-unsafely-without-email \
    -d example.com  \
    --rsa-key-size 4096 \
    --agree-tos \
    --force-renewal" certbot
