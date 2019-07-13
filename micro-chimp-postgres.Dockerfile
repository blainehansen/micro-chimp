FROM blainehansen:micro-chimp-node-codegen as node-codegen

ARG SITE_NAMES_FILE=site_names.yml

COPY ${SITE_NAMES_FILE} /build

WORKDIR /build

RUN node postgres-codegen.js $SITE_NAMES_FILE



FROM postgres:11-alpine

COPY --from=node-codegen /build/schema_site_name_enum.sql /docker-entrypoint-initdb.d/schema_0.sql

COPY ./schema.sql /docker-entrypoint-initdb.d/schema_1.sql
