pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }

    let mut nops = 0;
    let mut x = n;
    while x != 1 {
        match x & 1 {
            0 => x /= 2,
            1 => x = 3 * x + 1,
            _ => unreachable!(),
        }

        nops += 1;
    }

    Some(nops)
}
