use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();

    for c in candidate.chars().filter(|c| c.is_alphabetic()) {
        if !seen.insert(c.to_ascii_lowercase()) {
            return false;
        }
    }

    true
}
