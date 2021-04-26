const RADIX: u32 = 10;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if !code
        .chars()
        .all(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
    {
        return false;
    }

    if code.chars().filter(|c| c.is_ascii_digit()).count() <= 1 {
        return false;
    }

    let s: u32 = code
        .chars()
        .filter(|c| c.is_ascii_digit())
        .rev()
        .enumerate()
        .map(|(i, c)| match i % 2 {
            0 => c.to_digit(RADIX).unwrap(),
            _ => {
                let cx = c.to_digit(RADIX).unwrap();
                match 2 * cx {
                    x if x <= 9 => x,
                    x if x > 9 => x - 9,
                    _ => unreachable!(),
                }
            }
        })
        .sum();

    s % 10 == 0
}
