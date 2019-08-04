cp -R ~/.docker/machine/machines/micro-chimp . \
	&& mkdir ./micro-chimp/certs \
	&& cp -R ~/.docker/machine/certs/* ./micro-chimp/certs

sed -i.bak 's/machine\/certs/machine\/machines\/micro-chimp\/certs/' ./micro-chimp/config.json
sed -i.bak "s/$(whoami)/{{replace_username}}/" ./micro-chimp/config.json

tar -zcf .secret.micro-chimp.tar.gz ./micro-chimp
rm -Rf ./micro-chimp
