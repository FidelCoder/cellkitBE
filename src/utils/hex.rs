pub fn utf8_to_hex(content: &str) -> String {
    let mut output = String::from("0x");
    for byte in content.as_bytes() {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

pub fn normalize_hex(input: &str) -> Result<String, String> {
    let value = input.trim();
    let stripped = value.strip_prefix("0x").unwrap_or(value);

    if !stripped.len().is_multiple_of(2) {
        return Err("hex content must contain an even number of characters".to_string());
    }
    if !stripped.chars().all(|char| char.is_ascii_hexdigit()) {
        return Err("hex content contains non-hex characters".to_string());
    }

    Ok(format!("0x{}", stripped.to_ascii_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::{normalize_hex, utf8_to_hex};

    #[test]
    fn encodes_utf8_for_data_cells() {
        assert_eq!(utf8_to_hex("Hello CKB"), "0x48656c6c6f20434b42");
    }

    #[test]
    fn normalizes_hex_content() {
        assert_eq!(normalize_hex("0xCAFE").unwrap(), "0xcafe");
        assert!(normalize_hex("abc").is_err());
        assert!(normalize_hex("0xzz").is_err());
    }
}
