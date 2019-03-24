FROM node:11

COPY ./docker/rust-codegen.js /build/

COPY ./docker/postgres-codegen.js /build/

RUN npm install yaml
RUN npm install snake-case
