FROM tiangolo/uwsgi-nginx-flask:python3.7

RUN apt-get update
RUN apt-get install -y python3-numpy

ADD requirements.txt /app/requirements.txt
RUN pip install -r /app/requirements.txt


ENV NGINX_MAX_UPLOAD 0

COPY start-dev.sh /start-dev.sh
RUN chmod +x /start-dev.sh


COPY . /app
