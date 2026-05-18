export function total(items: Array<{ price: number; quantity: number }>): number {
  return items.reduce((sum, item) => sum + item.price * item.quantity, 0);
}

export function discountedTotal(
  items: Array<{ price: number; quantity: number }>,
  discount: number,
): number {
  return Math.max(0, total(items) - discount);
}
