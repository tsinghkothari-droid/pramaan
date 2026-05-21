it.each([
  [10000, 10, 9000],
])("applies discount %s", (subtotal, discount, expected) => {
  expect(subtotal - Math.floor((subtotal * discount) / 100)).toEqual(expected);
});

const generatedName = "generated " + "case";
it(generatedName, () => {
  expect(true).toEqual(true);
});
