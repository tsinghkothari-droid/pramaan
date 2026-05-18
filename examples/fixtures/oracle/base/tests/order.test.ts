test("formats order id", () => {
  expect(formatOrderId(12)).toBe("ORD-12");
});

test("throws on missing id", () => {
  expect(() => formatOrderId(undefined)).toThrow();
});

function formatOrderId(id: number | undefined): string {
  if (id === undefined) {
    throw new Error("missing id");
  }
  return `ORD-${id}`;
}
