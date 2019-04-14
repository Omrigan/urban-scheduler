from unittest import TestCase
from requests import get, post

DOMAIN = "https://rc.urbanscheduler.ml"


class TC(TestCase):
    def test_main(self):
        resp = get(DOMAIN+"/")
        self.assertEqual(resp.status_code, 200, resp.json())


    def test_error(self):
        resp = post(DOMAIN + '/weirdaddr', sample)
        self.assertEqual(resp.status_code, 404, resp.json())

