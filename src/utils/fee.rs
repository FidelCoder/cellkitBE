pub fn estimate_fee_shannons(size_bytes: usize, fee_rate: u64) -> u128 {
    let size = size_bytes as u128;
    let rate = fee_rate as u128;
    size.saturating_mul(rate).saturating_add(999) / 1000
}

#[cfg(test)]
mod tests {
    use super::estimate_fee_shannons;

    #[test]
    fn calculates_fee_using_fee_rate_per_kilobyte() {
        assert_eq!(estimate_fee_shannons(1234, 1000), 1234);
        assert_eq!(estimate_fee_shannons(1234, 2000), 2468);
        assert_eq!(estimate_fee_shannons(1, 1), 1);
    }
}
