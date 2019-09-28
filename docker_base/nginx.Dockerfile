FROM nginx:stable-alpine

COPY ./nginx.normal.conf /etc/nginx/includes/normal
COPY ./nginx.secure.conf /etc/nginx/includes/secure
