FROM postgres:11-alpine

COPY ./docker_base/postgres.schema.sql /docker-entrypoint-initdb.d/schema_1.sql
