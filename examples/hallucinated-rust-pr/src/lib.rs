use agent_magic_checkout::apply_discount;

pub fn total_after_discount(subtotal_cents: i64, percent: i64) -> i64 {
    apply_discount(subtotal_cents, percent)
}
