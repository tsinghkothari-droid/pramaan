from discount import apply_discount


def test_applies_percentage_discount():
    assert apply_discount(10000, 10) == 9000
