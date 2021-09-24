fn transform(c: char) -> char {
    if c.is_ascii_digit() {
        c
    } else {
        (b'z' - (c as u8 - b'a')) as char
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, c)| {
            if i > 0 && i % 5 == 0 {
                acc.push(' ')
            }

            acc.push(transform(c));

            acc
        })
        .into_iter()
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .fold(Vec::new(), |mut acc, c| {
            acc.push(transform(c));
            acc
        })
        .into_iter()
        .collect::<String>()
}
