import os
from collections import defaultdict
from copy import deepcopy
from datetime import datetime, timedelta

import geopy.distance
import numpy as np

from lib import cities

from process import connector
from exceptions import InvalidRequest, InvalidEvent, InvalidCity
from logic.event_types import EventTypes
import random
import osm_connector
import json
import datetime
import requests

RUST_URL = os.getenv('RUST_URL', "http://rustapp:80/predict_raw")


def to_rust_coords(python_location):
    return float(python_location['lat']), float(python_location['lng'])


def normalize_config(config):
    if config is None:
        config = {}
    result = {}
    city = config.get("city")
    if city not in cities:
        raise InvalidCity(city)

    result["city"] = city
    result['dists_method'] = config.get('routingBackend') or 'dummy'
    result["clipping"] = int(config.get('clipping') or 50)
    result["solver"] = config.get("solver") or "python"
    return result


def dummy_dist(x, y):
    return geopy.distance.distance((x['lat'], x['lng']), (y['lat'], y['lng'])).meters


def squash_distances(matrix1, matrix2):
    # print("Squashing", matrix1.shape, matrix2.shape)
    dists = np.zeros((matrix1.shape[0], matrix2.shape[1]), dtype=float)
    answers = np.zeros((matrix1.shape[0], matrix2.shape[1]), dtype=int)
    for i in range(dists.shape[0]):
        for j in range(dists.shape[1]):
            vector = matrix1[i] + matrix2[:, j]
            answers[i, j] = vector.argmin()
            dists[i, j] = vector[answers[i, j]]
    return dists, answers


db = connector.get_db()
places = db.places


class Predictor:
    def __init__(self, config=None):
        self.config = normalize_config(config)
        self.dists_method = self.config['dists_method']
        self.clipping = self.config['clipping']
        self.stages = []
        self.numbers_of_candidates = []
        self.start_time = None
        self.finish_time = None

    def start(self):
        self.start_time = datetime.datetime.now()

    def finish(self):
        self.finish_time = datetime.datetime.now()

    def checkpoint(self, name):
        td = datetime.datetime.now() - self.start_time
        self.stages.append((name, int(td.total_seconds() * 1000)))

    def any_place(self, event):
        candidates = self.get_candidates(event)
        if event['type'] == EventTypes.FIXED_PLACE:
            return event
        elif event['type'] == EventTypes.CATEGORY:
            candidate = random.choice(candidates)
            return places.find_one({'_id': candidate['_id']}, {'_id': 0})

    def calculate_distances(self, src_list, target_list):
        # print(DISTS, len(src_list), len(target_list))

        if self.dists_method == 'dummy':
            result = [[dummy_dist(s['location'], t['location']) for t in target_list] for s in src_list]
        elif self.dists_method == 'osrm':
            result = osm_connector.calcualte_distances("car", src_list, target_list)
        else:
            raise NotImplementedError("Dists %s" % self.dists_method)

        return np.asarray(result)

    def predict_ordered(self, ordered_events):
        self.start()
        ordered_events = [event for event in ordered_events if event.get('type') is not None]
        [self.validate_and_normalize(idx, event)
         for idx, event in enumerate(ordered_events)]

        if len(ordered_events) == 0:
            self.finish()
            return []
        if len(ordered_events) == 1:
            self.finish()
            return [self.any_place(ordered_events[0])]

        candidates = [self.get_candidates(event) for event in ordered_events]
        self.numbers_of_candidates = [len(item) for item in candidates]
        self.checkpoint("candidates_retrieved")
        if self.config['solver'] == "python":
            ordered_solution = self._solve_ordered(candidates)
        elif self.config['solver'] == "rust":
            ordered_solution = self._solve_ordered_rust(candidates)
        else:
            raise NotImplementedError

        self.checkpoint("ordered_solved")
        ordered_prediction = self.decode_ordered_ids(ordered_events, ordered_solution)
        self.checkpoint("finished")
        self.finish()
        return ordered_prediction

    def decode_ordered_ids(self, ordered_events, ordered_solution):
        result = []
        for i, id in enumerate(ordered_solution):
            if ordered_events[i]['type'] == 'fixed_place':
                result.append(ordered_events[i])
            else:
                result.append(places.find_one({'_id': id}, {'_id': 0}))
        return result

    def _solve_ordered_rust(self, all_candidates):
        events_int_to_mongo = []
        rust_events = []
        for event_idx, event in enumerate(all_candidates):
            this_dct = {}
            this_event = {
                "idx": event_idx,
                "points": []
            }
            for i, point in enumerate(event):
                this_dct[i] = point["_id"]
                this_event["points"].append({
                    "coords": to_rust_coords(point['location']),
                    "idx": i
                })
            events_int_to_mongo.append(this_dct)
            rust_events.append(this_event)

        self.checkpoint("rust_data_prepared")
        result = requests.post(RUST_URL, json={"ordered_events": rust_events,
                                               "config": self.config}).json()
        self.checkpoint("rust_completed")
        answer = []
        for dct, point in zip(events_int_to_mongo, result['schedule']):
            answer.append(dct[point['idx']])
        print(answer)
        return answer


    def _solve_ordered(self, all_candidates):
        answers = []  # 2..n
        current_dists = None

        for i in range(1, len(all_candidates)):
            last_dists = self.calculate_distances(all_candidates[i - 1], all_candidates[i])

            if current_dists is not None:
                current_dists, answer = squash_distances(current_dists, last_dists)
                answers.append(answer)
            else:
                current_dists = last_dists

        def myargmin(x):
            return np.unravel_index(np.argmin(x, axis=None), x.shape)

        self.checkpoint("ordered_forward_pass")

        start, end = myargmin(current_dists)
        revered_result_route = [end]
        current_point = end
        for i in reversed(range(0, len(answers))):
            current_answers = answers[i]
            next_point = current_answers[start, current_point]
            revered_result_route.append(next_point)
            current_point = next_point
        result_route = [start] + list(reversed(revered_result_route))

        idx_sequence = [all_candidates[i][point]['_id'] for i, point in enumerate(result_route)]
        return idx_sequence

    def validate_and_normalize(self, idx, event):
        if event['type'] not in EventTypes.all:
            raise InvalidEvent(event['type'], "unknown event")
        if event['type'] == EventTypes.FIXED_PLACE:
            if not event.get('name'):
                event['name'] = "Fixed place %s" % (idx + 1)
            if type(event.get("location")) != dict:
                raise InvalidEvent(event['type'], "fixed places must have location")

    def get_candidates(self, event):
        type = event.get('type')
        if type == 'fixed_place':
            return [{'location': event.get('location'), '_id': None}]
        elif type == 'category':
            category = event.get('category')
            brand = event.get('brand')
            query = {"categories": category, "city": self.config['city']}
            if brand:
                query['brand'] = brand
            result = list(places.find(query, {'location': 1}))

            if len(result) == 0:
                raise Exception("No places with this type %s" % event)
            if self.clipping:
                result = result[:self.clipping]
            return result
        return []

    def report(self):
        return {
            'stages': self.stages,
            "numbers_of_candidates": self.numbers_of_candidates,
            "config": self.config,
            'finish_time': self.finish_time.strftime("%c")
        }

    def center(self):
        return cities[self.config['city']]['center']
