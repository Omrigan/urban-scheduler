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

URL = "http://127.0.0.1:80"


class TC(TestCase):
    def _assert_exception(self, resp, exception):
        self.assertEqual(resp.get('error_code'), exception.code, resp)

    def test_params(self):
        resp = get(URL + "/get_params")

        self.assertEqual(resp.status_code, 200, resp.json())
        cats = resp.json()
        self.assertTrue(type(cats) == list)
        self.assertGreater(len(cats), 5)

    def test_error(self):
        sample = {
            "ordered_events": [
                {"type": "fixed_place"},
            ],
        }
        resp = post(URL + '/predict', json=sample).json()
        self._assert_exception(resp, InvalidEvent)

    def test_rust_consistent(self):
        p = Predictor({"solver": "rust", 'city': 'moscow'})
        prediction_rust = p.predict_ordered(sample_ordered)
        p = Predictor({"solver": "python", 'city': 'moscow'})
        prediction_python = p.predict_ordered(sample_ordered)
        self.assertListEqual(prediction_python, prediction_rust)

    def test_rust_same(self):
        f = open('ordered_benchmark.yaml')
        content = load(f)
        resp = post(URL + '/predict', json=content).json()
        print(resp)
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
