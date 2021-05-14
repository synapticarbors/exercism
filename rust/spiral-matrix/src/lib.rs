const DX: [isize; 4] = [0, 1, 0, -1];
const DY: [isize; 4] = [1, 0, -1, 0];

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let n = size as usize;
    let mut v = vec![vec![0; n]; n];

    if size == 0 {
        return v;
    }

    let mut x = 0;
    let mut y = -1;
    let mut z = 1;

    for i in 0..n + n - 1 {
        for _ in 0..(n + n - i) / 2 {
            x += DX[i % 4];
            y += DY[i % 4];
            v[x as usize][y as usize] = z;
            z += 1;
        }
    }

    v
}
