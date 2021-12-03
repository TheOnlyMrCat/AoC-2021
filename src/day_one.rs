use itertools::Itertools;

use crate::get_input;

pub fn run() {
    let input = get_input(1);
    let measurements = input.trim().split('\n').map(|s| s.trim().parse::<i32>().unwrap());

    let mut larger_count = 0;

    for ((a, b, c), (d, e, f)) in measurements.tuple_windows().tuple_windows() {
        if (d + e + f) > (a + b + c) {
            larger_count += 1;
        }
    }

    println!("{}", larger_count);
}