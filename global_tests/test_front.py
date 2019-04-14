from unittest import TestCase
from requests import get, post

DOMAIN = "https://rc.urbanscheduler.ml"


class TC(TestCase):
    def test_main(self):
        resp = get(DOMAIN+"/")
        self.assertEqual(resp.status_code, 200, resp.text)


    def test_error(self):
        resp = get(DOMAIN + '/weirdaddr')
        self.assertEqual(resp.status_code, 404, resp.text)

