#!/usr/bin/env bash
docker run --rm --link mongodb:mongo -v "/mongo-backup:/backup" mongo \
 bash -c ‘mongodump --out /backup --host mongo:27017’