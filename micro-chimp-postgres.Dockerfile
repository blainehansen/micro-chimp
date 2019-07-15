FROM node:11 as node-codegen

COPY ./docker/codegen.js /build/

RUN npm install yaml
RUN npm install snake-case

ARG SITE_NAMES_FILE=site_names.yml

COPY ${SITE_NAMES_FILE} /build

WORKDIR /build

RUN node codegen.js $SITE_NAMES_FILE


FROM postgres:11-alpine

COPY --from=node-codegen /build/schema_site_name_enum.sql /docker-entrypoint-initdb.d/schema_0.sql

COPY ./schema.sql /docker-entrypoint-initdb.d/schema_1.sql

