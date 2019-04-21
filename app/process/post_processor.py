import argparse
import os
from collections import defaultdict, Counter
import re
from lib import *
import pymongo
from process import connector

from pymongo import MongoClient

parser = argparse.ArgumentParser(description='tbd')

db = connector.get_db()


parser.add_argument('--city', type=str,
                    help='City', required=True)

args = parser.parse_args()

def insert_all(cats):
    collection = db['categories_' + args.city]
    collection.delete_many({})
    collection.insert_many(cats)


categories = defaultdict(lambda: {
    "brands": set(),
    "additional_fields": defaultdict(set)
})

places = db.places

for place in places.find({'city': args.city}):
    for cat in place["categories"]:
        categories[cat]["brands"].add(place['brand'])
        if "additional_fields" not in place:
            place["additional_fields"] = {}
            # place.save()
        for field_name, value in place["additional_fields"].items():
            categories[cat]["additional_fields"][field_name].add(value)

for cat, value in categories.items():
    value["name"] = cat
    value["brands"] = list(value["brands"])
    for field, options in value["additional_fields"].items():
        value["additional_fields"][field] = list(options)

insert_all(categories.values())