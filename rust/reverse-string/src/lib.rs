#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    #[cfg(feature = "grapheme")]
    let x = input.graphemes(true).rev().collect();

    #[cfg(not(feature = "grapheme"))]
    let x = input.chars().rev().collect();

    x
}
