import unittest

from checkout import discounted_total


class CheckoutTests(unittest.TestCase):
    def test_applies_percentage_discount(self):
        self.assertGreater(discounted_total(10_000, 10), 0)


if __name__ == "__main__":
    unittest.main()
