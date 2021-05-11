use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    // Algorithm from Right-to-left binary method in:
    // https://en.wikipedia.org/wiki/Modular_exponentiation
    // using u128 to handle large numbers
    if modulus == 1 {
        return 0;
    }

    let mut result = 1u128;
    let mut base = (base % modulus) as u128;
    let mut exp = exp as u128;
    let modulus = modulus as u128;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    result as u64
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}
