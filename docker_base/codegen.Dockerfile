FROM node

RUN npm install ts-node typescript yaml snake-case ts.data.json

COPY ../bin/codegen.ts
