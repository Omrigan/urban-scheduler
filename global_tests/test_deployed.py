from unittest import TestCase
from requests import get, post

DOMAIN = "https://api.rc.urbanscheduler.ml"


class TC(TestCase):
    def test_params(self):
        resp = get(DOMAIN+"/get_params")
        self.assertEqual(resp.status_code, 200, resp.json())


    def test_error(self):
        sample = {
            "ordered_events": [
                {"type": "fixed_place"},
            ]
        }
        resp = post(DOMAIN + '/predict', json=sample)
        self.assertEqual(resp.status_code, 400, resp.json())

