def total(items):
    return sum(price * quantity for price, quantity in items)


def discounted_total(items, discount):
    return max(0, total(items) - discount)
