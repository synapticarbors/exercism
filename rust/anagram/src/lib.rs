use std::collections::HashSet;

fn canonicalize_word(word: &str) -> Vec<char> {
    let mut x: Vec<char> = word.chars().collect();
    x.sort_unstable();

    x
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_canonical = canonicalize_word(&word_lower);

    let candidates_lower: Vec<String> =
        possible_anagrams.iter().map(|x| x.to_lowercase()).collect();

    let candidates_canonical: Vec<Vec<char>> = candidates_lower
        .iter()
        .map(|x| canonicalize_word(x))
        .collect();

    let mut anagrams = HashSet::new();
    let it = candidates_canonical.iter().zip(candidates_lower.iter());

    for (i, (cc, cl)) in it.enumerate() {
        if (*cl != word_lower) && (*cc == word_canonical) {
            anagrams.insert(possible_anagrams[i]);
        }
    }

    anagrams
}
