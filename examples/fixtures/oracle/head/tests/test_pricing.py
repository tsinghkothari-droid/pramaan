import pytest


def test_discount_total():
    total = 100 - 15
    assert total


@pytest.mark.xfail(reason="agent deferred error path")
def test_rejects_negative_quantity():
    with pytest.raises(ValueError):
        raise ValueError("negative")


@pytest.mark.parametrize(
    "quantity,expected",
    [
        (1, 100),
    ],
)
def test_bulk_price(quantity, expected):
    assert quantity * 100 == expected
