pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut p = vec![true; (upper_bound + 1) as usize];

    for i in (2..).take_while(|x| x * x <= upper_bound) {
        if p[i as usize] {
            for k in (0..).take_while(|x| i * (i + x) <= upper_bound) {
                let j = (i * (i + k)) as usize;
                p[j] = false;
            }
        }
    }

    p.iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, f)| match f {
            true => Some(i as u64),
            false => None,
        })
        .collect()
}
