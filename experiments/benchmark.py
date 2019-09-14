from unittest import TestCase
from requests import get, post
# from logic import PredictJob
from logic.predictor import Predictor
from exceptions import InvalidRequest, InvalidEvent
import json
from yaml import load
import numpy as np


def request(endpoint, data):
    return post("https://api.rc.urbanscheduler.ml" + endpoint, json=data).json()


def get_ordered_problem():
    f = open('problems/ordered_benchmark.yaml')
    content = load(f)
    return content


class TCPerformance(TestCase):
    def _measure(self, config):
        sample = get_ordered_problem()
        sample["config"] = config
        resp = request('/predict', sample)
        self.assertTrue('final_route' in resp)
        result_ms = resp['report']['stages'][-1][1]
        return result_ms

    def _measure_batch(self, config, size, comment=None):
        result = []
        for i in range(size):
            result.append(self._measure(config))
        print("Config:", config)
        if comment:
            print("Comment:", comment)
        print("Mean: %s. Std: %s" % (np.mean(result), np.std(result)))
        print('Full results:', result)

    def test_ordered(self):
        self._measure_batch({"solver": "python", 'clipping': 100, 'city': 'moscow'}, 10, "Python")
        self._measure_batch({"solver": "rust", 'clipping': 100, 'city': 'moscow'}, 10, "Rust nalgebra")
