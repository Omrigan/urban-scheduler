from unittest import TestCase
from requests import get, post
# from logic import PredictJob
from logic.predictor import Predictor
from exceptions import InvalidRequest, InvalidEvent
import json
from yaml import load


def request(endpoint, data):
    return post("https://api.rc.urbanscheduler.ml" + endpoint, json=data).json()

def get_ordered_problem():
    f = open('helpers/sample_requests/ordered_benchmark.yaml')
    content = load(f)
    return content


class TCPerformance(TestCase):
    def _measure(self, config):
        ordered_events = get_ordered_problem()
        sample = {
            "ordered_events": ordered_events,
            "config": config
        }
        resp = request('/predict', sample)
        self.assertTrue('final_route' in resp)
        result_ms = resp['report']['stages'][-1][1]
        return result_ms

    def test_ordered(self):
        print("Python result:", self._measure({"solver": "python", 'clipping': 5, 'city': 'moscow'}))
        print("Rust result:", self._measure({"solver": "rust", 'clipping': 5, 'city': 'moscow'}))