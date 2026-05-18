import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from order import render_order_summary


class OrderSummaryTest(unittest.TestCase):
    def test_order_summary_matches_snapshot(self):
        expected = (ROOT / "tests" / "__snapshots__" / "order.snap").read_text().strip()
        self.assertEqual(render_order_summary(), expected)


if __name__ == "__main__":
    unittest.main()
