use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .fold(HashSet::new(), |mut acc, c| {
            acc.insert(c);
            acc
        })
        .len()
        == 26
}
