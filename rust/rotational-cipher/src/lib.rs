pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let offset = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = key.rem_euclid(26) as u8;
                ((c as u8 - offset + shift) % 26 + offset) as char
            } else {
                c
            }
        })
        .collect()
}
