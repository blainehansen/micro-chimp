FROM postgres:11-alpine

COPY ./postgres.schema.sql /docker-entrypoint-initdb.d/schema_1.sql
