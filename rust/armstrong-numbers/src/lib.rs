fn num_digits(num: u32) -> u32 {
    let mut i = 0;
    let mut x = num;
    while x > 0 {
        x /= 10;
        i += 1
    }

    i
}

pub fn is_armstrong_number(num: u32) -> bool {
    let n = num_digits(num);
    let mut s = 0;
    let mut x = num;

    while x > 0 {
        let digit = x % 10;
        x /= 10;
        s += digit.pow(n);
    }

    s == num
}
