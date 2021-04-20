const ASCII_OFFSET: u8 = 65;
const CASE_WRAP: u8 = 32;

const SCORE_TABLE: [u64; 26] = [
    1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
];

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars()
        .map(|c| {
            let ci = c as u8;
            match ci {
                65..=90 | 97..=122 => SCORE_TABLE[((ci - ASCII_OFFSET) % CASE_WRAP) as usize],
                _ => 0,
            }
        })
        .sum()
}
