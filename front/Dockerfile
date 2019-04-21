FROM node as build

WORKDIR /front
ADD . /front

RUN npm install

RUN ls -lah


ARG REACT_APP_ENV
ARG REACT_APP_TITLE
ARG REACT_APP_URL

RUN echo $REACT_APP_ENV
RUN npm run build


FROM nginx:latest

COPY --from=build /front/build/ /usr/share/nginx/html/