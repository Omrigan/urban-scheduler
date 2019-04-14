#!/usr/bin/env bash
set -e

cd $1
# docker-compose -f docker-compose.yml run osrm rm -rf /data/*
docker-compose -f docker-compose.yml run osrm osrm-extract -p /opt/car.lua /data/osrm-final.osm.pbf
docker-compose -f docker-compose.yml run osrm osrm-partition /data/osrm-final.osrm
docker-compose -f docker-compose.yml run osrm osrm-customize /data/osrm-final.osrm