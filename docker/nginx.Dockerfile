FROM blainehansen/micro-chimp:codegen as codegen
COPY sites_manifest.yml .
RUN npx ts-node codegen.ts

FROM blainehansen/micro-chimp:nginx
COPY --from=codegen /home/nginx.conf /etc/nginx/conf.d/default.conf
