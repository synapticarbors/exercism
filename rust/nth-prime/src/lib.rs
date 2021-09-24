fn update_sieve(lower_bound: u32, upper_bound: u32, p: &mut [bool]) {
    for i in (lower_bound..).take_while(|x| x * x <= upper_bound) {
        if p[i as usize] {
            for k in (0..).take_while(|x| i * (i + x) <= upper_bound) {
                let j = (i * (i + k)) as usize;
                p[j] = false;
            }
        }
    }
}

fn get_primes(s: &[bool]) -> Vec<u32> {
    s.iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, f)| match f {
            true => Some(i as u32),
            false => None,
        })
        .collect()
}

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let lower_bound = 2;
    let mut upper_bound = n;
    let mut primes_found = -1;
    let mut s = vec![true; (upper_bound + 1) as usize];
    let mut p = vec![];

    while primes_found < (n as i32 + 1) {
        s.resize((upper_bound + 1) as usize, true);
        update_sieve(lower_bound, upper_bound, &mut s);
        p = get_primes(&s);
        primes_found = p.len() as i32;

        upper_bound *= 2;
    }

    p[n as usize]
}
