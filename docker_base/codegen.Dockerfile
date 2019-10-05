FROM node

WORKDIR /codegen

COPY bin/codegen.ts .
COPY tsconfig-bin.json .
COPY tsconfig.json .
COPY package.json .

RUN npm install
