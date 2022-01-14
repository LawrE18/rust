#![allow(unused)]
/*
In this exercise you'll be writing code to analyze the production of an assembly line in a car
factory. The assembly line's speed can range from 0 (off) to 10 (maximum).

At its lowest speed (1), 221 cars are produced each hour. The production increases linearly
with the speed. So with the speed set to 4, it should produce 4 * 221 = 884 cars per hour.
However, higher speeds increase the likelihood that faulty cars are produced, which then have
to be discarded. The following table shows how speed influences the success rate:
    1 to 4: 100% success rate.
    5 to 8: 90% success rate.
    9 and 10: 77% success rate.
 */

// My solution
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = if speed <= 4 {100.0} else if speed <= 8 {90.0} else {77.0};
    (speed as f64 * 221.0) * rate / 100.0
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}

// Community solution
pub fn production_rate_per_hour_(speed: u8) -> f64 {
    let success_rate = match speed {
        0..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => panic!("unexpected speed {}", speed)
    };

    (speed as f64) * 221.0 * success_rate
}

pub fn working_items_per_minute_(speed: u8) -> u32 {
    production_rate_per_hour_(speed) as u32 / 60
}