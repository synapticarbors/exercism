fn is_valid(nums: &[char]) -> bool {
    nums.iter().enumerate().all(|(i, c)| match i {
        0 | 3 => matches!(c, '2'..='9'),
        _ => true,
    })
}

pub fn number(user_number: &str) -> Option<String> {
    let nums: Vec<char> = user_number.chars().filter(char::is_ascii_digit).collect();

    match nums.len() {
        10 => match is_valid(&nums) {
            true => Some(nums.iter().collect()),
            false => None,
        },
        11 => match nums[0] == '1' && is_valid(&nums[1..]) {
            true => Some(nums[1..].iter().collect()),
            false => None,
        },
        _ => None,
    }
}
