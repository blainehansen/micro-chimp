FROM node:12

RUN npm install yaml
RUN npm install snake-case

COPY ./docker/codegen.js /generated/
