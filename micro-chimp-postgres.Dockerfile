FROM blainehansen:micro-chimp-node-codegen as node-codegen

ARG SITE_NAMES_FILE=site_names.yml

COPY ${SITE_NAMES_FILE} /build

WORKDIR /build

RUN node postgres-codegen.js $SITE_NAMES_FILE



FROM postgres:11-alpine

COPY ./init_0.sql ./init_2.sql /docker-entrypoint-initdb.d/

COPY --from=node-codegen /build/init.sql /docker-entrypoint-initdb.d/init_1.sql
