FROM node as build

WORKDIR /front
ADD . /front
RUN npm run build
FROM nginx:latest

COPY --from=build /front/build/ /usr/share/nginx/html/