import pytest


@pytest.mark.parametrize("subtotal,discount,expected", [(10000, 10, 9000)])
def test_parametrized_discount(subtotal, discount, expected):
    assert subtotal - ((subtotal * discount) // 100) == expected


def make_generated_test():
    def test_generated_runtime_case():
        assert True

    return test_generated_runtime_case
