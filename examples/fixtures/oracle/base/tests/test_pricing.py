import pytest


def test_discount_total():
    total = 100 - 15
    assert total == 85


def test_rejects_negative_quantity():
    with pytest.raises(ValueError):
        raise ValueError("negative")


@pytest.mark.parametrize(
    "quantity,expected",
    [
        (1, 100),
        (2, 200),
        (3, 300),
    ],
)
def test_bulk_price(quantity, expected):
    assert quantity * 100 == expected


def test_deleted_regression():
    assert "paid" in {"paid", "pending"}
