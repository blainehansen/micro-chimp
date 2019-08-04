docker build -t blainehansen/micro-chimp:nginx -f docker/nginx.Dockerfile .

docker build -t blainehansen/micro-chimp:postgres -f docker/postgres.Dockerfile .

docker build -t blainehansen/micro-chimp:rust -f docker/rust.Dockerfile .

docker build -t blainehansen/micro-chimp -f docker/micro-chimp.Dockerfile .
