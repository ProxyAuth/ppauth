import unittest
import time
import ppauth

class TestPyProxyAuth(unittest.TestCase):

    def setUp(self):
        self.host = "demo.proxyauth.app"
        self.port = 8080
        self.username = "admin"
        self.password = "admin123"
        self.timezone = "Europe/Paris"

    def test_initial_auth(self):
        ppauth.auth(
            host=self.host,
            port=self.port,
            username=self.username,
            password=self.password,
            timezone=self.timezone
        )
        token = ppauth.token()
        self.assertIsInstance(token, str)
        self.assertGreater(len(token), 10)

    def test_token_cache(self):
        ppauth.auth(
            host=self.host,
            port=self.port,
            username=self.username,
            password=self.password,
            timezone=self.timezone

        )
        token1 = ppauth.token()
        time.sleep(1)
        token2 = ppauth.token()
        self.assertEqual(token1, token2)

    def test_token_renew(self):
        ppauth.auth(
            host=self.host,
            port=self.port,
            username=self.username,
            password=self.password,
            timezone=self.timezone
        )
        token1 = ppauth.token()
        lease = ppauth.lease_token()
        time.sleep(lease + 1)
        token2 = ppauth.token(renew=True)
        self.assertNotEqual(token1, token2)

    def test_token_expiration(self):
        ppauth.auth(
            host=self.host,
            port=self.port,
            username=self.username,
            password=self.password,
            timezone=self.timezone
        )
        token1 = ppauth.token()
        time.sleep(5)
        token2 = ppauth.token()
        self.assertEqual(token1, token2)

    def test_remaining_time_decreases(self):
        ppauth.auth(
            host=self.host,
            port=self.port,
            username=self.username,
            password=self.password,
            timezone=self.timezone
        )
        time1 = ppauth.lease_token()
        time.sleep(1)
        time2 = ppauth.lease_token()
        self.assertLess(time2, time1)

if __name__ == '__main__':
    unittest.main()
