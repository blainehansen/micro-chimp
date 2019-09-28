FROM nginx:stable-alpine

COPY ./docker_base/nginx.normal.conf /etc/nginx/includes/normal
COPY ./docker_base/nginx.secure.conf /etc/nginx/includes/secure
