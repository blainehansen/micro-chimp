FROM nginx:stable-alpine

COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY ./nginx.normal.conf /etc/nginx/includes/normal
COPY ./nginx.secure.conf /etc/nginx/includes/secure
