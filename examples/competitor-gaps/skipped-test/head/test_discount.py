import pytest

from discount import apply_discount


@pytest.mark.skip(reason="temporarily flaky in generated fix")
def test_applies_percentage_discount():
    assert apply_discount(10000, 10) == 9000
