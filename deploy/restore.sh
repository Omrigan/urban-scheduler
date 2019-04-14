#!/usr/bin/env bash
cd $1
docker-compose exec mongo bash -c "mongorestore --drop -d cityday  /mongo_ready/$2/cityday"