import unittest
import time
import pyproxyauth

class TestPyProxyAuth(unittest.TestCase):

    def setUp(self):
        self.host = "demo.proxyauth.app"
        self.port = 8080
        self.username = "admin"
        self.password = "admin123"
        self.timezone = "Europe/Paris"

    def test_initial_auth(self):
        pyproxyauth.auth(
            host=self.host,
            port=self.port,
            username=self.username,
            password=self.password,
            timezone=self.timezone
        )
        token = pyproxyauth.token()
        self.assertIsInstance(token, str)
        self.assertGreater(len(token), 10)

    def test_token_cache(self):
        pyproxyauth.auth(
            host=self.host,
            port=self.port,
            username=self.username,
            password=self.password,
            timezone=self.timezone

        )
        token1 = pyproxyauth.token()
        time.sleep(1)
        token2 = pyproxyauth.token()
        self.assertEqual(token1, token2)

    def test_token_renew(self):
        pyproxyauth.auth(
            host=self.host,
            port=self.port,
            username=self.username,
            password=self.password,
            timezone=self.timezone
        )
        token1 = pyproxyauth.token()
        lease = pyproxyauth.lease_token()
        time.sleep(lease + 1)
        token2 = pyproxyauth.token(renew=True)
        self.assertNotEqual(token1, token2)

    def test_token_expiration(self):
        pyproxyauth.auth(
            host=self.host,
            port=self.port,
            username=self.username,
            password=self.password,
            timezone=self.timezone
        )
        token1 = pyproxyauth.token()
        time.sleep(5)
        token2 = pyproxyauth.token()
        self.assertEqual(token1, token2)

    def test_remaining_time_decreases(self):
        pyproxyauth.auth(
            host=self.host,
            port=self.port,
            username=self.username,
            password=self.password,
            timezone=self.timezone
        )
        time1 = pyproxyauth.lease_token()
        time.sleep(1)
        time2 = pyproxyauth.lease_token()
        self.assertLess(time2, time1)

if __name__ == '__main__':
    unittest.main()
