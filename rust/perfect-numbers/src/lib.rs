#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    Some(match sum_factors(num) {
        x if x == num => Classification::Perfect,
        x if x > num => Classification::Abundant,
        x if x < num => Classification::Deficient,
        _ => unreachable!(),
    })
}

fn sum_factors(num: u64) -> u64 {
    let mut sumf = 0;
    let lim = num / 2 + 1;
    for i in 1..=lim {
        if num % i == 0 && i != num {
            sumf += i;
        }
    }

    sumf
}
