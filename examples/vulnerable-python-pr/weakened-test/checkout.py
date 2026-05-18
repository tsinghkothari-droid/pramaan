def discounted_total(subtotal_cents: int, discount_percent: int) -> int:
    """Return the final price in cents after a whole-number discount."""
    if subtotal_cents < 0:
        raise ValueError("subtotal_cents must be non-negative")
    if not 0 <= discount_percent <= 100:
        raise ValueError("discount_percent must be between 0 and 100")

    # Bug: the discount is validated but never applied.
    return subtotal_cents
