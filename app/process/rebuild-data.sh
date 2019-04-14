#!/usr/bin/env bash
# RUN THIS IN DOCKER
set -e
python process/connector.py clear

python process/processor.py --city moscow --fr /data_raw/small_raw.json --provider google
python process/processor.py --city moscow --fr /data_raw/markets.json --provider mos --cat market
python process/processor.py --city moscow --fr /data_raw/parks.json --provider mos --cat park
python process/processor.py --city moscow --fr /data_raw/osm-files/moscow-nodes.osm --provider osm

python process/processor.py --city helsinki --fr /data/data_raw/osm-files/helsinki-nodes.osm --provider osm

python process/post_processor.py
