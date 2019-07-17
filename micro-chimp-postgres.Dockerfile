FROM blainehansen/micro-chimp:codegen as codegen

ARG SITE_NAMES_FILE=site_names.yml

COPY ${SITE_NAMES_FILE} /generated

WORKDIR /generated

RUN node codegen.js $(basename $SITE_NAMES_FILE)


FROM postgres:11-alpine

COPY --from=codegen /generated/schema_site_name_enum.sql /docker-entrypoint-initdb.d/schema_0.sql

COPY ./schema.sql /docker-entrypoint-initdb.d/schema_1.sql

