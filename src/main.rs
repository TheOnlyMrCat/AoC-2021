mod day_one;
mod day_two;
mod day_three;
mod day_four;

fn get_input(day: u8) -> String {
    std::fs::read_to_string(format!("input/day{}.txt", day)).unwrap()
}

fn main() {
    match std::env::args().nth(1).and_then(|s| s.parse::<u8>().ok()) {
        Some(1) => day_one::run(),
        Some(2) => day_two::run(),
        Some(3) => day_three::run(),
        _ => day_four::run(),
    }
}