#!/usr/bin/env bash
set -e
cp ../data/osm-files/osrm-final.osm.pbf ../data/osrm
docker-compose -f dev/docker-compose.yml run osrm rm -rf /data/*
docker-compose -f dev/docker-compose.yml run osrm osrm-extract -p /opt/car.lua /data/osrm-final.osm.pbf
docker-compose -f dev/docker-compose.yml run osrm osrm-partition /data/osrm-final.osrm
docker-compose -f dev/docker-compose.yml run osrm osrm-customize /data/osrm-final.osrm