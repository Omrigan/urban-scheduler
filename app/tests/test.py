from unittest import TestCase
from requests import get, post
# from logic import PredictJob
from logic.predictor import Predictor
from exceptions import InvalidRequest, InvalidEvent
import json

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


def request(endpoint, data):
    return post("http://127.0.0.1:80" + endpoint, json=data).json()

class TC(TestCase):
    def _assert_exception(self, resp, exception):
        self.assertEqual(resp.get('error_code'), exception.code, resp)

    def test_predict_ordered(self):
        p = Predictor()
        prediction = p.predict_ordered(sample_ordered)
        self.assertEqual(prediction[0], sample_ordered[0])
        self.assertEqual(prediction[1]['brand'], "даблби")
        # print(prediction)

    def test_predict_ordered_2(self):
        sample = {
            "ordered_events": [
                {"type": "fixed_place", "location": {"lat": 55.76896444850637, "lng": 37.63229370117188}},
                {"type": "category", "category": "park"},
                {"type": "category", "category": "cafe", "brand": "шоколадница"},
                {"type": "fixed_place", "location": {"lat": 55.759499939832814, "lng": 37.64602661132813}},
                {"type": None}]
        }
        p = Predictor()
        prediction = p.predict_ordered(sample["ordered_events"])
        self.assertEqual(len(prediction), 4)

    def test_performance(self):
        p = Predictor(dict(clipping=100))
        prediction = p.predict_ordered(sample_ordered)
        stages = p.report()['stages']
        finished = stages[-1]
        self.assertEqual(finished[0], 'finished')
        self.assertLess(finished[1], 5000)  # 5 seconds

    def test_params(self):
        resp = get("http://127.0.0.1:80/get_params")
        self.assertEqual(resp.status_code, 200)
        # print(resp.json())

    def test_error(self):
        sample = {
            "ordered_events": [
                {"type": "fixed_place"},
            ]
        }
        resp = request('/predict', sample)
        self._assert_exception(resp, InvalidEvent)
