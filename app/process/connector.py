import argparse
import os
from collections import defaultdict, Counter
import re
from lib import *
import pymongo
import time
from pymongo import MongoClient


def get_db():
    while True:
        try:
            client = MongoClient('mongo', 27017)
            info = client.server_info()  # Forces a call.
            break
        except ServerSelectionTimeoutError:
            print("server is down.")
            time.sleep(5)

    db = client.cityday
    return db



if __name__=="__main__":
    parser = argparse.ArgumentParser(description='Import process.')
    parser.add_argument("command")

    args = parser.parse_args()

    db = get_db()
    if args.command=='clear':
        db.places.delete_many({})
        db.categories.delete_many({})
        print("Cleared")
