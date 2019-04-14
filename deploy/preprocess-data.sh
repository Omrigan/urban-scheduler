#!/usr/bin/env bash
set -e

cd $1

docker-compose exec app process/rebuild-data.sh
