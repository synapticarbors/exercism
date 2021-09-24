use std::cmp::Ordering;

pub fn is_palindrome(n: u64) -> bool {
    let mut num = n;
    let mut rev = 0;

    while num > 0 {
        rev = rev * 10 + (num % 10);
        num /= 10;
    }

    rev == n
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            value: a.saturating_mul(b),
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut min_p = Palindrome::new(u64::MAX, u64::MAX);
    let mut max_p = Palindrome::new(0, 0);

    for a in min..=max {
        for b in a..=max {
            let p = a * b;
            if is_palindrome(p) {
                match p.cmp(&min_p.value) {
                    Ordering::Less => min_p = Palindrome::new(a, b),
                    Ordering::Equal => min_p.insert(a, b),
                    _ => (),
                }

                match p.cmp(&max_p.value) {
                    Ordering::Greater => max_p = Palindrome::new(a, b),
                    Ordering::Equal => max_p.insert(a, b),
                    _ => (),
                }
            }
        }
    }

    if min_p.value != u64::MAX && max_p.value != 0 {
        Some((min_p, max_p))
    } else {
        None
    }
}
