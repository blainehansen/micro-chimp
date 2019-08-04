FROM postgres:11-alpine

COPY ./site_name_enum.sql /docker-entrypoint-initdb.d/schema_0.sql
COPY ./postgres.schema.sql /docker-entrypoint-initdb.d/schema_1.sql

