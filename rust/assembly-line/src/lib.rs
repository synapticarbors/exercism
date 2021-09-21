// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASE_CARS_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    BASE_CARS_PER_HOUR
        * (speed as f64)
        * match speed {
            0..=4 => 1.0,
            5..=8 => 0.9,
            9 | 10 => 0.77,
            _ => unreachable!(),
        }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
