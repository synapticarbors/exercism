use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'T' | 'C' | 'G' => (),
        _ => return Err(nucleotide),
    }

    let mut count = 0;
    for c in dna.chars() {
        match c {
            c if c == nucleotide => count += 1,
            c if c == 'A' || c == 'T' || c == 'C' || c == 'G' => (),
            _ => return Err(c),
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        "ACTG"
            .chars()
            .map(|c| (c, 0usize))
            .collect::<HashMap<_, _>>(),
        |mut acc, c| {
            match c {
                'A' | 'T' | 'C' | 'G' => {
                    let e = acc.entry(c).or_default();
                    *e += 1;
                }
                _ => return Err(c),
            }

            Ok(acc)
        },
    )
}
