FROM blainehansen/micro-chimp:codegen as codegen
COPY sites_manifest.yml .
RUN npx ts-node codegen.ts

FROM blainehansen/micro-chimp:postgres
COPY --from=codegen /codegen/site_name_enum.sql /docker-entrypoint-initdb.d/schema_0.sql
