//! DEC-003 / CTR-04: content IDs. `OP-` + Crockford base32 of the BLAKE3 digest, shown as `OP-XXXX-XXXX`.

const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Crockford base32 of the first `chars * 5` bits of `bytes`.
fn base32(bytes: &[u8], chars: usize) -> String {
    let mut out = String::with_capacity(chars);
    for i in 0..chars {
        let bit = i * 5;
        let (byte, offset) = (bit / 8, bit % 8);
        let hi = *bytes.get(byte).unwrap_or(&0) as u16;
        let lo = *bytes.get(byte + 1).unwrap_or(&0) as u16;
        let v = (((hi << 8) | lo) >> (11 - offset)) & 0x1f;
        out.push(ALPHABET[v as usize] as char);
    }
    out
}

/// The display ID for a digest: 8 chars normally, `long` = 12 chars when two different contents collide.
pub fn format_id(digest: &[u8; 32], long: bool) -> String {
    let chars = if long { 12 } else { 8 };
    let s = base32(digest, chars);
    let groups: Vec<&str> = (0..chars).step_by(4).map(|i| &s[i..i + 4]).collect();
    format!("OP-{}", groups.join("-"))
}

/// Normalise what someone types as a reference: drop `OP`, dashes, spaces; upper case; Crockford aliases.
pub fn normalize_query(q: &str) -> String {
    let up = q.trim().to_ascii_uppercase();
    let up = up.strip_prefix("OP-").or_else(|| up.strip_prefix("OP")).unwrap_or(&up).to_string();
    up.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| match c {
            'O' => '0',
            'I' | 'L' => '1',
            other => other,
        })
        .collect()
}

/// The ID without `OP-` and dashes, for matching against `normalize_query`.
pub fn compact(id: &str) -> String {
    id.trim_start_matches("OP-").replace('-', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_stable_for_fixed_content() {
        // IDX-01-A01: this value must never change (CTR-04).
        let d = *blake3::hash(b"hola\n").as_bytes();
        let id = format_id(&d, false);
        assert_eq!(id.len(), 12);
        assert!(id.starts_with("OP-"));
        assert_eq!(id, format_id(&d, false));
        assert_eq!(id, FIXED_HOLA_ID);
        assert_eq!(format_id(&d, true).len(), 17);
    }

    const FIXED_HOLA_ID: &str = "OP-D3CV-TA7N";

    #[test]
    fn query_normalisation() {
        assert_eq!(normalize_query("op-7k3m-q9xa"), "7K3MQ9XA");
        assert_eq!(normalize_query(" 7K3M "), "7K3M");
        assert_eq!(normalize_query("OP-1O1L"), "1011");
        assert_eq!(compact("OP-7K3M-Q9XA"), "7K3MQ9XA");
    }
}
