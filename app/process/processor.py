import argparse
import os
from collections import defaultdict, Counter
import re
from lib import *
import pymongo
from process import connector

from pymongo import MongoClient

import xml.etree.ElementTree as et

parser = argparse.ArgumentParser(description='Import process.')

parser.add_argument('--fr', type=str,
                    help='From file', required=True)

parser.add_argument('--cat', type=str,
                    help='Category')

parser.add_argument('--provider', type=str,
                    help='Provider of source process', required=True)


parser.add_argument('--city', type=str,
                    help='City', required=True)

args = parser.parse_args()

expr = re.compile("(\s+|-)")


def get_brand(name):
    if not name:
        return None
    return re.sub(expr, " ", name.lower())


def insert_all(places):
    db = connector.get_db()
    collection = db.places
    for place in places:
        place["city"] = args.city
    collection.insert_many(places)


all_places = []

if args.provider == 'google':
    raw_places = json.load(open(args.fr))
    for place in raw_places:
        place_ready = {
            "name": place["name"],
            "location": place["geometry"]["location"],
            "brand": get_brand(place["name"]),
            "categories": place["types"],
            "additional_fields": {}
        }
        all_places.append(place_ready)

elif args.provider == 'mos':

    raw_places = json.load(open(args.fr, encoding='cp1251'))
    assert args.cat is not None
    cat = args.cat
    for place in raw_places:
        if args.cat == 'park':
            name = place['CommonName']
            loc = place['geoData']["center"][0]
            description = place.get("Location")
            fields_list = ["HasPlayground",
                           "HasSportground",
                           "HasWater",
                           "NeighbourhoodPark"]
            place_ready = {
                "name": name,
                "description": description,
                "location": {
                    'lng': loc[0],
                    'lat': loc[1]
                },
                "brand": get_brand(name),
                "categories": [args.cat]
            }
            additional_fields = {field: place.get(field) for field in fields_list}

            place_ready["additional_fields"] = additional_fields
        elif args.cat == 'market':
            name = place.get('Name')
            loc = place['geoData']['coordinates']

            market_type = place.get("MarketType")

            place_ready = {
                "name": name,
                "additional_fields": {"market_type": market_type},
                "location": {
                    'lng': loc[0],
                    'lat': loc[1]
                },
                "brand": get_brand(name),
                "categories": [args.cat]
            }
        else:
            raise NotImplementedError()
        all_places.append(place_ready)
elif args.provider == 'osm':


    mapping = {
        ('amenity', 'cafe'): 'cafe',
        ('amenity', 'fast_food'): 'fastfood',
        ('amenity', 'food_court'): 'fastfood',
        ('amenity', 'bar'): 'bar',
        ('amenity', 'pub'): 'bar',
        ('amenity', 'restaurant'): 'restaurant',
        ('amenity', 'atm'): 'atm',
        ('amenity', 'bank'): 'bank',
        ('shop', 'supermarket'): 'supermarket',
        ('shop', 'wholesale'): 'supermarket',
        ('shop', 'convenience'): 'supermarket',

    }
    e = et.parse(args.fr).getroot()
    for n in e.iter('node'):
        loc = {'lat': n.attrib['lat'], 'lng': n.attrib['lon']}
        tags = {}
        for tag in n:
            tag_key = tag.attrib['k'].replace(".", "_")
            tags[tag_key] = tag.attrib['v']

        if 'name' in tags:
            # print(tags)
            place_ready = dict(tags)
            place_ready.update({
                "location": loc,
                "brand": get_brand(tags["name"]),
                "categories": []
            })
            if 'brand' in tags:
                place_ready["original_brand"] = tags['brand']

            for predicate, cat in mapping.items():

                if tags.get(predicate[0]) == predicate[1]:
                    # if predicate[1] == 'cafe':
                    #     print(place_ready)
                    place_ready["categories"].append(cat)

            if len(place_ready["categories"]) == 0:
                place_ready["categories"].append("other")
            all_places.append(place_ready)
else:
    raise NotImplementedError("Provider unknown %s" % args.provider)

print("Inserting to db")
insert_all(all_places)
print("Inserted")
