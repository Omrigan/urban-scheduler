import argparse
import os
from collections import defaultdict, Counter
import re
from lib import *
import pymongo
from process import connector

from pymongo import MongoClient

import xml.etree.ElementTree as et
import requests
import toml, json, yaml

parser = argparse.ArgumentParser(description='Import process.')

parser.add_argument('--file', type=str,
                    help='Input data', required=True)

args = parser.parse_args()


def submit(data):
    result = requests.post("http://localhost:80/predict", json=data)
    print(result.text)
    return result.json()


filename = args.file
content = open(filename).read()
print(content)
if filename[-4:] == 'toml':
    task = toml.loads(content)
elif filename[-4:] == 'json':
    task = json.loads(content)
else:
    raise NotImplementedError
result = submit(task)
print("resulting_dist", result.get("resulting_dist"))
print()
for item in result["schedule"]:
    for k, v in item.items():
        print(k, v)
    print()
