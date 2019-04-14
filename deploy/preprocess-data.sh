#!/usr/bin/env bash
set -e
docker-compose -f dev/docker-compose.yml exec app process/rebuild-data.sh
