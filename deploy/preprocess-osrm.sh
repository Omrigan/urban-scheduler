#!/usr/bin/env bash
cp ../data/osm-files/moscow.osm.pbf ../data/osrm
docker-compose -f dev/docker-compose.yml run osrm osrm-extract -p /opt/car.lua /data/moscow.osm.pbf
docker-compose -f dev/docker-compose.yml run osrm osrm-partition /data/moscow.osrm
docker-compose -f dev/docker-compose.yml run osrm osrm-customize /data/moscow.osrm