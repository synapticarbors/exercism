pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let nrows = minefield.len();

    if nrows == 0 {
        return Vec::new();
    }

    let ncols = minefield[0].len();

    if ncols == 0 {
        return vec!["".to_string(); nrows];
    }

    let counts = minefield.iter().flat_map(|x| x.chars()).enumerate().fold(
        vec![0; nrows * ncols],
        |mut acc, (ix, c)| {
            if c == '*' {
                let i = ix % nrows;
                let j = ix / nrows;

                // Set location of mine to -9 so that it is guaranteed
                // to be negative after looking at all neighbors
                acc[ix] -= 9;

                for di in -1..=1 {
                    for dj in -1..=1 {
                        if (di == 0) && (dj == 0) {
                            continue;
                        }

                        let ni = i as isize + di;
                        let nj = j as isize + dj;

                        if (ni < 0) || (ni >= nrows as isize) {
                            continue;
                        }

                        if (nj < 0) || (nj >= ncols as isize) {
                            continue;
                        }

                        acc[(ni + nrows as isize * nj) as usize] += 1;
                    }
                }
            }

            acc
        },
    );

    // Convert counts to output format
    counts
        .chunks(ncols)
        .map(|row| {
            row.iter()
                .map(|&cnt| match cnt {
                    0 => ' '.to_string(),
                    cnt if cnt < 0 => '*'.to_string(),
                    _ => cnt.to_string(),
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect()
}
