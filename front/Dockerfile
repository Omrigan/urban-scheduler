FROM node as build

WORKDIR /front

ARG REACT_APP_ENV
ARG REACT_APP_TITLE
ARG REACT_APP_URL

ADD . /front

RUN npm install

RUN npm run build

FROM nginx:latest

COPY --from=build /front/build/ /usr/share/nginx/html/