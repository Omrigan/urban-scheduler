FROM debian

RUN apt-get update  && \
    apt-get install -y liblapack-dev gcc libgfortran4

COPY ./SCIPOptSuite-6.0.2-Linux/bin /usr/local/bin

RUN scip

