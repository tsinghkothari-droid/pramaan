it("parser ignores comment and string noise", () => {
  const note = "expect(total).toEqual({ skipped: true })";
  // it.skip("not a real skip")
  expect(total)
    .toEqual({
      cents: 4200
    });
});
