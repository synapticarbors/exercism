/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let x: Vec<u32> = isbn
        .chars()
        .filter(|c| c.is_digit(10) || *c == 'X')
        .map(|w| if let Some(y) = w.to_digit(10) { y } else { 10 })
        .collect();

    if x.len() != 10 || (x.contains(&10) && x.last() != Some(&10)) {
        return false;
    }

    x.iter()
        .enumerate()
        .map(|(i, xn)| (10 - i as u32) * xn)
        .sum::<u32>()
        .rem_euclid(11)
        == 0
}
