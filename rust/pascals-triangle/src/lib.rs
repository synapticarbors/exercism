pub struct PascalsTriangle {
    _rows: Vec<Vec<u32>>,
}

/// Calculate the binomial coefficient (n choose k) based on p = (n choose k -1)
fn binomial_coeff(n: u32, k: u32, p: u32) -> u32 {
    p * (n + 1 - k) / k
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = Vec::with_capacity(row_count as usize);

        for n in 0..row_count as usize {
            let mut row = Vec::with_capacity(n + 1);
            // (n choose 0) is always 1
            row.push(1);

            for k in 1..n + 1 {
                row.push(binomial_coeff(n as u32, k as u32, row[k - 1]));
            }

            rows.push(row);
        }

        PascalsTriangle { _rows: rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self._rows.to_vec()
    }
}
