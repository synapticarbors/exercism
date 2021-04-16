pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let nrows = input.len();
    let ncols = input[0].len();

    let mut saddlepoints = Vec::new();

    // Find max value in each row
    let row_max: Vec<&u64> = input
        .iter()
        .map(|row| row.iter().max().unwrap_or(&0))
        .collect();

    // Find min value in each column
    let col_min: Vec<&u64> = input.iter().fold(vec![&u64::MAX; ncols], |mut acc, row| {
        for i in 0..ncols {
            if row[i] <= *acc[i] {
                acc[i] = &row[i]
            }
        }
        acc
    });

    for i in 0..nrows {
        for j in 0..ncols {
            let this_val = input[i][j];

            if (this_val >= *row_max[i]) && (this_val <= *col_min[j]) {
                saddlepoints.push((i, j));
            }
        }
    }

    saddlepoints
}
