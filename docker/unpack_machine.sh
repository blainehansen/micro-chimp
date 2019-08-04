tar -xzf .secret.micro-chimp.tar.gz --directory ~/.docker/machines/micro-chimp

sed -i.bak "s/{{replace_username}}/$(whoami)/" ~/.docker/machines/micro-chimp/config.json

eval $(docker-machine env micro-chimp)
