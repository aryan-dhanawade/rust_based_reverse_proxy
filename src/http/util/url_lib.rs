/// Percent‑encode a string following RFC 3986 (unreserved chars are
/// ALPHA / DIGIT / "-" / "." / "_" / "~").
pub fn url_encode(input: &str) -> String {
    // Unreserved characters per RFC 3986
    const UNRESERVED: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                               abcdefghijklmnopqrstuvwxyz\
                               0123456789-._~";
    const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

    let bytes = input.as_bytes();

    let mut out = String::with_capacity(bytes.len() * 3);

    for &b in bytes {
        if UNRESERVED.contains(&b) {
            // safe to unwrap: b is valid UTF‑8
            out.push(b as char);
        } else {
            out.push('%');
            out.push(HEX_DIGITS[(b >> 4) as usize] as char);
            out.push(HEX_DIGITS[(b & 0x0F) as usize] as char);
        }
    }

    out
}


pub fn url_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;

    /// Convert a single hex digit (ASCII) to its value.
    #[inline]
    fn hex_val(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'A'..=b'F' => Some(b - b'A' + 10),
            b'a'..=b'f' => Some(b - b'a' + 10),
            _ => None,
        }
    }

    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hi = hex_val(bytes[i + 1]);
                let lo = hex_val(bytes[i + 2]);
                if let (Some(h), Some(l)) = (hi, lo) {
                    out.push((h << 4) | l);
                    i += 3;
                } else {
                    // malformed %—emit '%' and continue
                    out.push(b'%');
                    i += 1;
                }
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }

    // Convert bytes → String, replacing invalid UTF‑8 with U+FFFD
    String::from_utf8(out).unwrap_or_else(|v| String::from_utf8_lossy(v.as_bytes()).into())
}

