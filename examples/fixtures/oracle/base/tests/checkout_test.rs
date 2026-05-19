fn price_for_quantity(quantity: i32) -> Result<i32, &'static str> {
    if quantity < 0 {
        return Err("negative");
    }
    Ok(quantity * 100)
}

#[test]
fn discounts_bulk_quantity() {
    assert_eq!(price_for_quantity(3).unwrap(), 300);
}

#[test]
fn rejects_negative_quantity() {
    assert!(price_for_quantity(-1).is_err());
}

#[test]
#[should_panic]
fn panics_on_missing_fixture() {
    panic!("missing fixture");
}

#[test]
fn deleted_rust_regression() {
    assert_eq!(price_for_quantity(0).unwrap(), 0);
}
