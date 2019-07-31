docker-machine create --driver=digitalocean \
	--digitalocean-access-token=$(tr -d "[:space:]" < .secret.digital_ocean_key) \
	--digitalocean-image=coreos-stable \
	--digitalocean-region=sfo2 \
	--digitalocean-size=1GB \
	--digitalocean-ssh-user=core \
	micro-chimp
