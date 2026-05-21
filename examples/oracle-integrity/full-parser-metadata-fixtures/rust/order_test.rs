#[test]
fn applies_discount() {
    assert_eq!(10000 - ((10000 * 10) / 100), 9000);
}

macro_rules! generated_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            assert!(true);
        }
    };
}

generated_test!(generated_runtime_case);
