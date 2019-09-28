FROM blainehansen/micro-chimp:codegen as codegen

# ARG TYPE='postgres'
RUN ts-node codegen.ts

FROM postgres:11-alpine

COPY --from=codegen ./site_name_enum.sql /docker-entrypoint-initdb.d/schema_0.sql
COPY ./postgres.schema.sql /docker-entrypoint-initdb.d/schema_1.sql
