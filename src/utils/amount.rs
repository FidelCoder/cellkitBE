const SHANNONS_PER_CKB: u128 = 100_000_000;

pub fn ckb_to_shannons(input: &str) -> Result<u128, String> {
    let value = input.trim();
    if value.is_empty() {
        return Err("amountCkb is required".to_string());
    }
    if value.starts_with('-') {
        return Err("amountCkb must be greater than 0".to_string());
    }

    let parts: Vec<&str> = value.split('.').collect();
    if parts.len() > 2 {
        return Err("amountCkb must be a valid decimal number".to_string());
    }

    let whole = parts[0];
    if whole.is_empty() || !whole.chars().all(|char| char.is_ascii_digit()) {
        return Err("amountCkb must be a valid decimal number".to_string());
    }

    let fractional = parts.get(1).copied().unwrap_or("");
    if !fractional.chars().all(|char| char.is_ascii_digit()) {
        return Err("amountCkb must be a valid decimal number".to_string());
    }
    if fractional.len() > 8 {
        return Err("amountCkb supports at most 8 decimal places".to_string());
    }

    let whole_shannons = whole
        .parse::<u128>()
        .map_err(|_| "amountCkb is too large".to_string())?
        .checked_mul(SHANNONS_PER_CKB)
        .ok_or_else(|| "amountCkb is too large".to_string())?;

    let mut fractional_padded = fractional.to_string();
    while fractional_padded.len() < 8 {
        fractional_padded.push('0');
    }
    let fractional_shannons = if fractional_padded.is_empty() {
        0
    } else {
        fractional_padded
            .parse::<u128>()
            .map_err(|_| "amountCkb must be a valid decimal number".to_string())?
    };

    let total = whole_shannons
        .checked_add(fractional_shannons)
        .ok_or_else(|| "amountCkb is too large".to_string())?;

    if total == 0 {
        return Err("amountCkb must be greater than 0".to_string());
    }

    Ok(total)
}

pub fn shannons_to_ckb(shannons: u128) -> String {
    let whole = shannons / SHANNONS_PER_CKB;
    let fractional = shannons % SHANNONS_PER_CKB;

    if fractional == 0 {
        return whole.to_string();
    }

    let mut fractional_text = format!("{fractional:08}");
    while fractional_text.ends_with('0') {
        fractional_text.pop();
    }

    format!("{whole}.{fractional_text}")
}

#[cfg(test)]
mod tests {
    use super::{ckb_to_shannons, shannons_to_ckb};

    #[test]
    fn converts_ckb_to_shannons() {
        assert_eq!(ckb_to_shannons("100").unwrap(), 10_000_000_000);
        assert_eq!(ckb_to_shannons("0.00000001").unwrap(), 1);
        assert_eq!(ckb_to_shannons("1.23").unwrap(), 123_000_000);
    }

    #[test]
    fn converts_shannons_to_ckb() {
        assert_eq!(shannons_to_ckb(10_000_000_000), "100");
        assert_eq!(shannons_to_ckb(1), "0.00000001");
        assert_eq!(shannons_to_ckb(123_000_000), "1.23");
    }

    #[test]
    fn rejects_zero_and_negative_amounts() {
        assert!(ckb_to_shannons("0").is_err());
        assert!(ckb_to_shannons("0.00000000").is_err());
        assert!(ckb_to_shannons("-1").is_err());
    }
}
