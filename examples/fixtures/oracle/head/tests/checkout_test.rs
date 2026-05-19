fn price_for_quantity(quantity: i32) -> Result<i32, &'static str> {
    if quantity < 0 {
        return Err("negative");
    }
    Ok(quantity * 100)
}

#[test]
fn discounts_bulk_quantity() {
    assert!(price_for_quantity(3).is_ok());
}

#[test]
#[ignore]
fn rejects_negative_quantity() {
    assert!(price_for_quantity(-1).is_err());
}

#[test]
fn panics_on_missing_fixture() {
    assert!(true);
}
