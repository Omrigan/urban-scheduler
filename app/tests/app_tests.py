from unittest import TestCase
from requests import get, post
# from logic import PredictJob
from logic.predictor import Predictor
from exceptions import InvalidRequest, InvalidEvent
import json

from yaml import load

sample_ordered = [
    {
        "type": "fixed_place",
        "location": {'lat': 55.7494539, 'lng': 37.62160470000001, },
        "finish_time": "15:00"
    },
    {
        "type": "category",
        "category": "cafe",
        "brand": "даблби",
        "delay": 15
    },
    {

        "type": "category",
        "category": "park",
        "delay": 60
    },
    {

        "type": "category",
        "category": "restaurant",
        "delay": 20
    },
    {
        "type": "fixed_place",
        "start_time": "23:00",
        "location": {'lat': 55.7494539, 'lng': 37.62160470000001, },
    }
]


class TC(TestCase):
    def test_predict_ordered(self):
        p = Predictor({'city': 'moscow'})
        prediction = p.predict_ordered(sample_ordered)
        self.assertEqual(prediction[0], sample_ordered[0])
        self.assertEqual(prediction[1]['brand'], "даблби")


    def test_predict_ordered_2(self):
        sample = {
            "ordered_events": [
                {"type": "fixed_place", "location": {"lat": 55.76896444850637, "lng": 37.63229370117188}},
                {"type": "category", "category": "park"},
                {"type": "category", "category": "cafe", "brand": "шоколадница"},
                {"type": "fixed_place", "location": {"lat": 55.759499939832814, "lng": 37.64602661132813}},
                {"type": None}]
        }
        p = Predictor({'city': 'moscow'})
        prediction = p.predict_ordered(sample["ordered_events"])
        self.assertEqual(len(prediction), 4)

    def test_performance(self):
        p = Predictor(dict(clipping=100, city='moscow'))
        prediction = p.predict_ordered(sample_ordered)
        stages = p.report()['stages']
        finished = stages[-1]
        self.assertEqual(finished[0], 'finished')
        self.assertLess(finished[1], 10000)  # 10 seconds
