use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|x: char| !x.is_ascii_alphanumeric() && x != '\'')
        .map(|w| {
            w.to_ascii_lowercase()
                .trim_matches(|x: char| x.is_ascii_punctuation())
                .to_string()
        })
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut acc, w| {
            *acc.entry(w).or_insert(0) += 1;
            acc
        })
}
