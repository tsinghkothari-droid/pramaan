test("formats order id", () => {
  expect(formatOrderId(12)).toBeTruthy();
});

test.todo("throws on missing id");

function formatOrderId(id: number | undefined): string {
  if (id === undefined) {
    throw new Error("missing id");
  }
  return `ORD-${id}`;
}
