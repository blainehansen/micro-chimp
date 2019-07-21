docker build -t blainehansen/micro-chimp:codegen -f docker/codegen.Dockerfile .

docker build -t blainehansen/micro-chimp:postgres -f docker/postgres.Dockerfile .

docker build -t blainehansen/micro-chimp:rust -f docker/rust.Dockerfile .

docker build -t blainehansen/micro-chimp -f docker/micro-chimp.Dockerfile .
