#![allow(dead_code)]

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
mod day_nine;

fn get_input(day: u8) -> String {
    std::fs::read_to_string(format!("input/day{}.txt", day)).unwrap()
}

fn main() {
    match std::env::args().nth(1).and_then(|s| s.parse::<u8>().ok()) {
        Some(1) => day_one::run(),
        Some(2) => day_two::run(),
        Some(3) => day_three::run(),
        Some(4) => day_four::run(),
        Some(5) => day_five::run(),
        Some(6) => day_six::run(),
        Some(7) => day_seven::run(),
        Some(8) => day_eight::run(),
        _ => day_nine::run(),
    }
}