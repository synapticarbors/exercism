pub fn is_leap_year(year: u64) -> bool {
    let is_div_4 = year % 4 == 0;
    let is_div_100 = year % 100 == 0;
    let is_div_400 = year % 400 == 0;

    matches!(
        (is_div_4, is_div_100, is_div_400),
        (true, false, true) | (true, false, false) | (true, true, true)
    )
}
