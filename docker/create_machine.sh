docker-machine create --driver=digitalocean \
	--digitalocean-access-token=$(tr -d "[:space:]" < .secret.digital_ocean_key) \
	--digitalocean-image=coreos-stable \
	--digitalocean-region=sfo2 \
	--digitalocean-size=1GB \
	--digitalocean-ssh-user=core \
	micro-chimp

cp -R ~/.docker/machine/machines/micro-chimp . \
	&& mkdir ./micro-chimp/certs \
	&& cp -R ~/.docker/machine/certs/* ./micro-chimp/certs

sed -i.bak 's/machine\/certs/machine\/machines\/micro-chimp\/certs/' ./micro-chimp/config.json
sed -i.bak "s/$(whoami)/{{replace_username}}/" ./micro-chimp/config.json

tar -zcf .secret.micro-chimp.tar.gz ./micro-chimp
git secret add .secret.micro-chimp.tar.gz
rm -Rf ./micro-chimp
