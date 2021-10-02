use std::iter;

pub fn encrypt(input: &str) -> String {
    let normalized: String = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let n = normalized.len();
    let c = (n as f64).sqrt().ceil() as usize;
    let r = if c * c == n { c } else { c - 1 };

    let n_target = c * r;

    let mut tmp = [0; 4];

    normalized
        .chars()
        .chain(iter::repeat(' '))
        .take(n_target)
        .enumerate()
        .fold(vec!["".to_string(); c], |mut acc, (i, ch)| {
            acc[i % c].push_str(ch.encode_utf8(&mut tmp));
            acc
        })
        .join(" ")
}
