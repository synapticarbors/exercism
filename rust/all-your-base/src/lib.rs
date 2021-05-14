#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn to_decimal(number: &[u32], base: u32) -> Result<u32, Error> {
    number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| match d {
            d if d < &base => Ok(d * base.pow(i as u32)),
            _ => Err(Error::InvalidDigit(*d)),
        })
        .sum()
}

fn decimal_to_base(value: u32, base: u32) -> Vec<u32> {
    if value == 0 {
        return vec![0];
    }

    let mut out = Vec::new();
    let mut v = value;

    while v > 0 {
        out.push(v % base);
        v /= base;
    }

    out.reverse();
    out
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    // Check valid numbers
    for d in number.iter() {
        if *d >= from_base {
            return Err(Error::InvalidDigit(*d));
        }
    }

    let value = to_decimal(number, from_base)?;

    Ok(decimal_to_base(value, to_base))
}
