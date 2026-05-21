def apply_discount(subtotal_cents: int, percentage: int) -> int:
    if percentage < 0 or percentage > 100:
        raise ValueError("percentage must be between 0 and 100")
    return subtotal_cents - ((subtotal_cents * percentage) // 100)
