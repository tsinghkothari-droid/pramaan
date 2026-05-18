pub fn total(items: &[(u32, u32)]) -> u32 {
    items.iter().map(|(price, quantity)| price * quantity).sum()
}

pub fn discounted_total(items: &[(u32, u32)], discount: u32) -> u32 {
    total(items).saturating_sub(discount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_discount() {
        assert_eq!(discounted_total(&[(10, 2)], 5), 15);
    }
}
