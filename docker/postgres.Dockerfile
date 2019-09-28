FROM postgres:11-alpine

# FROM blainehansen/micro-chimp:postgres

COPY ./site_name_enum.sql /docker-entrypoint-initdb.d/schema_0.sql
COPY ./postgres.schema.sql /docker-entrypoint-initdb.d/schema_1.sql
