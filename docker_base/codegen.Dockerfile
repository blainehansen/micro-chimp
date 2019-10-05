FROM node

WORKDIR home
RUN npm init -y

COPY bin/codegen.ts .
COPY tsconfig-bin.json .
COPY tsconfig.json .
COPY package.json .

RUN npm install
