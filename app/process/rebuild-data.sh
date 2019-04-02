#!/usr/bin/env bash
# RUN THIS IN DOCKER
set -e
#python process/connector.py clear
#python process/processor.py --fr data/raw_data/small_raw.json --provider google
#python process/processor.py --fr data/raw_data/markets.json --provider mos --cat market
#python process/processor.py --fr data/raw_data/parks.json --provider mos --cat park
#python process/processor.py --fr data/osm-files/moscow-nodes.osm --provider osm
python process/post_processor.py
#python processor.py --fr mos_data/markets.json --to mos-markets.json --provider mos --cat market
#python processor.py --fr mos_data/parks.json --to mos-parks.json --provider mos --cat park --upsert_brands
#python processor.py --fr parsed_data/small_raw.json --to google-small.json --provider google --upsert_brands
#python processor.py --fr osm-files/moscow-nodes.osm --to osm-all.json --provider osm  --upsert_brands
