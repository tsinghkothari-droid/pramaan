from checkout import discounted_total


def test_discounted_total():
    assert discounted_total([(10, 2)], 5) == 15
