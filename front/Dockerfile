FROM node as build

WORKDIR /front
ADD . /front

FROM nginx:latest

COPY --from=build /front/build/ /usr/share/nginx/html/