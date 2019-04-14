#!/usr/bin/env bash
cd dev
docker-compose exec mongo bash -c "mongodump -d cityday --out /mongo_backups/$@"