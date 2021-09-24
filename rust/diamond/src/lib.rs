use std::iter;

pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as u8 - b'A' + 1;
    let w = (2 * n - 1) as usize;

    let bottom_half = ('A'..=c)
        .rev()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, x)| {
            let mut tmp: String = iter::repeat(' ').take(w).collect();
            let r = x.to_string();
            tmp.replace_range(i..i + 1, &r);
            tmp.replace_range(w - i - 1..w - i, &r);
            acc.push(tmp);
            acc
        });

    let mut d: Vec<String> = bottom_half[1..].iter().cloned().rev().collect();
    d.extend(bottom_half);

    d
}
