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


def request(endpoint, data):
    return post("http://127.0.0.1:80" + endpoint, json=data).json()


class TC(TestCase):
    def _assert_exception(self, resp, exception):
        self.assertEqual(resp.get('error_code'), exception.code, resp)

    def test_predict_ordered(self):
        p = Predictor({'city': 'moscow'})
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

    def test_params(self):
        resp = get("http://127.0.0.1:80/get_params")
        self.assertEqual(resp.status_code, 200, resp.json())
        # print(resp.json())

    def test_error(self):
        sample = {
            "ordered_events": [
                {"type": "fixed_place"},
            ],
        }
        resp = request('/predict', sample)
        self._assert_exception(resp, InvalidEvent)

    def test_rust_consistent(self):
        p = Predictor({"solver": "rust", 'city': 'moscow'})
        prediction_rust = p.predict_ordered(sample_ordered)
        p = Predictor({"solver": "python", 'city': 'moscow'})
        prediction_python = p.predict_ordered(sample_ordered)
        self.assertListEqual(prediction_python, prediction_rust)

    def test_rust_same(self):
        f = open('helpers/sample_requests/ordered_benchmark.yaml')
        content = load(f)
        resp = request('/predict', content)
        print(resp['schedule'])
        self.assertEqual(resp['schedule'], [{
                                                'additional_fields': {}, 'brand': 'музейный парк',
                                                'categories': ['park', 'point_of_interest', 'establishment'],
                                                'city': 'moscow',
                                                'location': {'lat': 55.7589777, 'lng': 37.62769979999999},
                                                'name': 'Музейный парк'
                                            }, {
                                                'amenity': 'cafe', 'brand': 'шоколадница', 'categories': ['cafe'],
                                                'city': 'moscow',
                                                'contact:facebook': 'https://www.facebook.com/shoko.ru',
                                                'contact:instagram': 'https://www.instagram.com/shoko.ru',
                                                'contact:ok': 'https://ok.ru/shokoru',
                                                'contact:twitter': 'https://twitter.com/wwwShokoRu',
                                                'contact:vk': 'https://vk.com/shokoru',
                                                'contact:website': 'http://shoko.ru', 'cuisine': 'coffee_shop',
                                                'diet:vegetarian': 'no',
                                                'location': {'lat': '55.757364', 'lng': '37.6316441'},
                                                'name': 'Шоколадница', 'name:en': 'Shokoladnitsa',
                                                'name:ru': 'Шоколадница', 'opening_hours': '24/7',
                                                'original_brand': 'Шоколадница', 'phone': '+7 985 310-46-32',
                                                'wheelchair': 'no'
                                            }, {
                                                'additional_fields': {}, 'brand': 'ресторан "marea"',
                                                'categories': ['restaurant', 'food', 'point_of_interest',
                                                               'establishment'], 'city': 'moscow',
                                                'location': {'lat': 55.7566465, 'lng': 37.6323381},
                                                'name': 'Ресторан "Marea"'
                                            }]
                         )

        # self.assertEqual('final_route' in resp)
