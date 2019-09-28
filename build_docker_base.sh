docker build -t blainehansen/micro-chimp:codegen -f docker_base/codegen.Dockerfile .
docker push blainehansen/micro-chimp:codegen

docker build -t blainehansen/micro-chimp:nginx -f docker_base/nginx.Dockerfile .
docker push blainehansen/micro-chimp:nginx

docker build -t blainehansen/micro-chimp:postgres -f docker_base/postgres.Dockerfile .
docker push blainehansen/micro-chimp:postgres

docker build -t blainehansen/micro-chimp:rust -f docker_base/rust.Dockerfile .
docker push blainehansen/micro-chimp:rust
