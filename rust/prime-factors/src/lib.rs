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

pub fn factors(n: u64) -> Vec<u64> {
    let mut f = Vec::new();
    let mut n = n;

    let limit = (n as f64).sqrt() as u64;
    let primes = primes_up_to(limit);

    for d in primes.iter() {
        while n % d == 0 {
            f.push(*d);
            n /= d;
        }
    }

    if n > 1 {
        f.push(n);
    }

    f
}
